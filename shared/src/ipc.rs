use anyhow::{Context, Result};
use std::path::PathBuf;
use std::net::TcpStream;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::time::Duration;
use tokio::net::{TcpStream as TokioTcpStream, UnixListener, UnixStream as TokioUnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;
use std::sync::Arc;
use directories;

use crate::models::IpcMessage;

const IPC_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB

// Helper to determine if a path is a Unix socket path
fn is_unix_socket_path(addr: &str) -> bool {
    addr.starts_with('/') || addr.contains('/')
}

#[derive(Debug)]
pub struct IpcServer {
    listener: Arc<UnixListener>,
    address: String,
}

impl IpcServer {
    pub fn new(socket_path: PathBuf) -> Result<Self> {
        let socket_path_str = socket_path.to_string_lossy().to_string();

        // Remove the socket file if it already exists
        if socket_path.exists() {
            std::fs::remove_file(&socket_path)
                .with_context(|| format!("Failed to remove existing socket at {:?}", socket_path))?;
        }

        let listener = UnixListener::bind(&socket_path)
            .with_context(|| format!("Failed to bind to Unix socket at {:?}", socket_path))?;

        Ok(IpcServer {
            listener: Arc::new(listener),
            address: socket_path_str,
        })
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn create_new() -> Result<Self> {
        // Use the XDG config directory for the socket
        let proj_dirs = directories::ProjectDirs::from("", "", "orion")
            .context("Failed to get project directories")?;

        let config_dir = proj_dirs.config_dir();

        // Ensure the directory exists
        std::fs::create_dir_all(config_dir)
            .with_context(|| format!("Failed to create config directory at {:?}", config_dir))?;

        let socket_path = config_dir.join("orion.sock");
        Self::new(socket_path)
    }

    pub async fn start_async(&self) -> Result<()> {
        loop {
            let (mut socket, _) = self.listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = vec![0; MAX_MESSAGE_SIZE];
                if let Ok(n) = socket.read(&mut buf).await {
                    if n > 0 {
                        if let Ok(message) = serde_json::from_slice::<IpcMessage>(&buf[..n]) {
                            // Handle message here
                            let response = serde_json::to_vec(&message)?;
                            socket.write_all(&response).await?;
                        }
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
        }
    }

    pub async fn receive_message(&self) -> Result<IpcMessage> {
        let (mut socket, _) = self.listener.accept().await?;
        let mut buf = vec![0; MAX_MESSAGE_SIZE];
        let n = socket.read(&mut buf).await?;
        let message = serde_json::from_slice::<IpcMessage>(&buf[..n])?;
        Ok(message)
    }
}

#[derive(Debug)]
pub enum IpcClientStream {
    Tcp(TcpStream),
    Unix(UnixStream),
}

impl Read for IpcClientStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            IpcClientStream::Tcp(stream) => stream.read(buf),
            IpcClientStream::Unix(stream) => stream.read(buf),
        }
    }
}

impl Write for IpcClientStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            IpcClientStream::Tcp(stream) => stream.write(buf),
            IpcClientStream::Unix(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            IpcClientStream::Tcp(stream) => stream.flush(),
            IpcClientStream::Unix(stream) => stream.flush(),
        }
    }
}

#[derive(Debug)]
pub struct IpcClient {
    stream: IpcClientStream,
}

impl IpcClient {
    pub fn new(server_addr: &str) -> Result<Self> {
        // Determine if this is a Unix socket path or TCP address
        if is_unix_socket_path(server_addr) {
            let stream = UnixStream::connect(server_addr)
                .with_context(|| format!("Failed to connect to Unix socket at {}", server_addr))?;

            Ok(IpcClient { stream: IpcClientStream::Unix(stream) })
        } else {
            let stream = TcpStream::connect(server_addr)
                .with_context(|| format!("Failed to connect to TCP server at {}", server_addr))?;

            Ok(IpcClient { stream: IpcClientStream::Tcp(stream) })
        }
    }

    pub fn send_message(&mut self, message: &IpcMessage) -> Result<()> {
        let serialized = serde_json::to_vec(message)?;
        if serialized.len() > MAX_MESSAGE_SIZE {
            return Err(anyhow::anyhow!("Message too large: {} bytes", serialized.len()));
        }

        self.stream.write_all(&serialized)?;
        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<IpcMessage> {
        let mut buffer = vec![0; MAX_MESSAGE_SIZE];
        let bytes_read = self.stream.read(&mut buffer)?;

        if bytes_read > 0 {
            let message: IpcMessage = serde_json::from_slice(&buffer[..bytes_read])?;
            Ok(message)
        } else {
            Err(anyhow::anyhow!("Connection closed by server"))
        }
    }

    pub async fn send_message_async(&mut self, message: &IpcMessage) -> Result<()> {
        let serialized = serde_json::to_vec(message)?;
        if serialized.len() > MAX_MESSAGE_SIZE {
            return Err(anyhow::anyhow!("Message too large: {} bytes", serialized.len()));
        }

        match &self.stream {
            IpcClientStream::Tcp(tcp_stream) => {
                let mut stream = TokioTcpStream::from_std(tcp_stream.try_clone()?)?;
                timeout(IPC_TIMEOUT, stream.write_all(&serialized)).await??;
            },
            IpcClientStream::Unix(_) => {
                // For Unix sockets, we'll just use the synchronous API
                // as it's more reliable across platforms
                self.stream.write_all(&serialized)?;
            }
        }

        Ok(())
    }

    pub async fn receive_message_async(&mut self) -> Result<IpcMessage> {
        let mut buffer = vec![0; MAX_MESSAGE_SIZE];

        let bytes_read = match &self.stream {
            IpcClientStream::Tcp(tcp_stream) => {
                let mut stream = TokioTcpStream::from_std(tcp_stream.try_clone()?)?;
                timeout(IPC_TIMEOUT, stream.read(&mut buffer)).await??
            },
            IpcClientStream::Unix(_) => {
                // For Unix sockets, we'll just use the synchronous API
                self.stream.read(&mut buffer)?
            }
        };

        if bytes_read > 0 {
            let message: IpcMessage = serde_json::from_slice(&buffer[..bytes_read])?;
            Ok(message)
        } else {
            Err(anyhow::anyhow!("Connection closed by server"))
        }
    }

    pub fn connect_to_default() -> Result<Self> {
        // Use the XDG config directory for the socket
        let proj_dirs = directories::ProjectDirs::from("", "", "orion")
            .context("Failed to get project directories")?;

        let config_dir = proj_dirs.config_dir();
        let socket_path = config_dir.join("orion.sock").to_string_lossy().to_string();

        Self::new(&socket_path)
    }

    pub fn get_address(&self) -> Option<String> {
        match &self.stream {
            IpcClientStream::Tcp(stream) => {
                stream.peer_addr().ok().map(|addr| addr.to_string())
            },
            IpcClientStream::Unix(stream) => {
                // For Unix sockets, we don't have a direct way to get the path
                // but we can return a best guess based on what we connected to
                None
            }
        }
    }
}
