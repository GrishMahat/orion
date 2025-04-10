use anyhow::{Context, Result};
use std::path::PathBuf;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;
use tokio::net::{TcpStream as TokioTcpStream, UnixListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;

use crate::models::IpcMessage;

const IPC_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB

#[derive(Debug)]
pub struct IpcServer {
    listener: UnixListener,
    address: String,
}

impl IpcServer {
    pub fn new(_socket_path: PathBuf) -> Result<Self> {
        let listener = UnixListener::bind("background.sock")
            .with_context(|| "Failed to bind to Unix socket")?;
        Ok(IpcServer {
            listener,
            address: "background.sock".to_string(),
        })
    }

    pub fn address(&self) -> String {
        self.address.clone()
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
pub struct IpcClient {
    stream: TcpStream,
}

impl IpcClient {
    pub fn new(server_addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(server_addr)
            .with_context(|| format!("Failed to connect to IPC server at {}", server_addr))?;
        
        Ok(IpcClient { stream })
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
        
        let mut stream = TokioTcpStream::from_std(self.stream.try_clone()?)?;
        timeout(IPC_TIMEOUT, stream.write_all(&serialized)).await??;
        Ok(())
    }

    pub async fn receive_message_async(&mut self) -> Result<IpcMessage> {
        let mut stream = TokioTcpStream::from_std(self.stream.try_clone()?)?;
        let mut buffer = vec![0; MAX_MESSAGE_SIZE];
        
        let bytes_read = timeout(IPC_TIMEOUT, stream.read(&mut buffer)).await??;
        
        if bytes_read > 0 {
            let message: IpcMessage = serde_json::from_slice(&buffer[..bytes_read])?;
            Ok(message)
        } else {
            Err(anyhow::anyhow!("Connection closed by server"))
        }
    }
} 