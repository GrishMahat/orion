use anyhow::{Context, Result};
use shared::{ipc, models, logging};
use std::process::{Command, Child};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;
use directories;

#[derive(Debug)]
pub struct ProcessManager {
    popup_process: Arc<Mutex<Option<Child>>>,
    ipc_client: Arc<Mutex<ipc::IpcClient>>,
    max_retries: u32,
    retry_delay: Duration,
}

impl ProcessManager {
    pub fn new(server_addr: &str) -> Result<Self> {
        Ok(ProcessManager {
            popup_process: Arc::new(Mutex::new(None)),
            ipc_client: Arc::new(Mutex::new(ipc::IpcClient::new(server_addr)?)),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
        })
    }

    pub async fn start_popup(&self) -> Result<()> {
        let mut process = self.popup_process.lock().await;

        if process.is_none() {
            logging::info("Starting popup UI process");

            // Get IPC socket address to pass to the popup UI
            let client = self.ipc_client.lock().await;
            let ipc_addr = match &client.get_address() {
                Some(addr) => addr.clone(),
                None => {
                    // If not available, try to get from config
                    let proj_dirs = directories::ProjectDirs::from("", "", "orion")
                        .context("Failed to get project directories")?;
                    let config_dir = proj_dirs.config_dir();
                    config_dir.join("orion.sock").to_string_lossy().to_string()
                }
            };

            // Try to find the popup_ui executable in several locations
            let executable_paths = [
                "popup_ui".to_string(),                       // In PATH
                "./popup_ui".to_string(),                     // Current directory
                "./target/release/popup_ui".to_string(),      // Release build
                "./target/debug/popup_ui".to_string(),        // Debug build
                "./dist/bin/popup_ui".to_string(),            // Distribution directory
            ];

            logging::info(&format!("Trying to start popup_ui with socket: {}", ipc_addr));

            let mut success = false;
            for path in &executable_paths {
                logging::info(&format!("Trying to start from path: {}", path));

                match Command::new(path)
                    .arg(&ipc_addr)  // Pass the socket path as an argument
                    .spawn()
                {
                    Ok(child) => {
                        *process = Some(child);
                        logging::info(&format!("Popup UI process started successfully from {}", path));
                        success = true;
                        break;
                    }
                    Err(e) => {
                        logging::warn(&format!("Failed to start from {}: {}", path, e));
                    }
                }
            }

            if !success {
                return Err(anyhow::anyhow!("Failed to start popup UI from any known location"));
            }

            // Wait for process to initialize
            sleep(Duration::from_millis(500)).await;
        } else {
            logging::warn("Popup UI is already running");
        }

        Ok(())
    }

    pub async fn stop_popup(&self) -> Result<()> {
        let mut process = self.popup_process.lock().await;

        if let Some(mut child) = process.take() {
            logging::info("Stopping popup UI process");

            child.kill()
                .with_context(|| "Failed to kill popup UI")?;

            match child.wait() {
                Ok(status) => {
                    logging::info(&format!("Popup UI process stopped with status: {}", status));
                }
                Err(e) => {
                    logging::error(&format!("Error waiting for popup UI process: {}", e));
                }
            }
        } else {
            logging::warn("Popup UI is not running");
        }

        Ok(())
    }

    pub async fn send_message(&self, message: models::IpcMessage) -> Result<()> {
        let mut retries = 0;
        let mut client = self.ipc_client.lock().await;

        loop {
            match client.send_message_async(&message).await {
                Ok(_) => {
                    logging::debug(&format!("Message sent successfully: {:?}", message));
                    return Ok(());
                }
                Err(e) => {
                    retries += 1;
                    if retries >= self.max_retries {
                        return Err(e).with_context(|| "Failed to send message after maximum retries");
                    }

                    logging::warn(&format!(
                        "Failed to send message (attempt {}): {}",
                        retries,
                        e
                    ));

                    sleep(self.retry_delay).await;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub async fn receive_message(&self) -> Result<models::IpcMessage> {
        let mut retries = 0;
        let mut client = self.ipc_client.lock().await;

        loop {
            match client.receive_message_async().await {
                Ok(message) => {
                    logging::debug(&format!("Message received successfully: {:?}", message));
                    return Ok(message);
                }
                Err(e) => {
                    retries += 1;
                    if retries >= self.max_retries {
                        return Err(e).with_context(|| "Failed to receive message after maximum retries");
                    }

                    logging::warn(&format!(
                        "Failed to receive message (attempt {}): {}",
                        retries,
                        e
                    ));

                    sleep(self.retry_delay).await;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub async fn restart_popup(&self) -> Result<()> {
        logging::info("Restarting popup UI process");

        self.stop_popup().await?;
        sleep(Duration::from_millis(100)).await;
        self.start_popup().await?;

        logging::info("Popup UI process restarted successfully");
        Ok(())
    }

    pub async fn is_popup_running(&self) -> bool {
        let process = self.popup_process.lock().await;
        process.is_some()
    }
}
