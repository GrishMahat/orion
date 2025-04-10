use anyhow::{Context, Result};
use shared::{ipc, models, logging};
use std::process::{Command, Child};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;

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
            
            let child = Command::new("popup_ui")
                .spawn()
                .with_context(|| "Failed to start popup UI")?;
            
            *process = Some(child);
            logging::info("Popup UI process started successfully");
            
            // Wait for process to initialize
            sleep(Duration::from_millis(100)).await;
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