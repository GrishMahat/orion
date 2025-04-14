use anyhow::{Result, Context};
use shared::models::{Command, Action};
use std::process;
use std::path::Path;

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, command: &Command) -> Result<()> {
        match &command.action {
            Action::OpenFile(path) => self.open_file(path),
            Action::ExecuteCommand(cmd) => self.execute_shell_command(cmd),
            Action::OpenUrl(url) => self.open_url(url),
            Action::Custom(custom) => {
                // For now, just log that we received a custom command
                println!("Custom command received: {}", custom);
                Ok(())
            }
        }
    }

    fn open_file(&self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy();

        #[cfg(target_os = "windows")]
        {
            process::Command::new("explorer")
                .arg(path)
                .spawn()
                .with_context(|| format!("Failed to open file: {}", path_str))?;
        }

        #[cfg(target_os = "macos")]
        {
            process::Command::new("open")
                .arg(path)
                .spawn()
                .with_context(|| format!("Failed to open file: {}", path_str))?;
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            process::Command::new("xdg-open")
                .arg(path)
                .spawn()
                .with_context(|| format!("Failed to open file: {}", path_str))?;
        }

        Ok(())
    }

    fn execute_shell_command(&self, command: &str) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            process::Command::new("cmd")
                .arg("/C")
                .arg(command)
                .spawn()
                .with_context(|| format!("Failed to execute command: {}", command))?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .spawn()
                .with_context(|| format!("Failed to execute command: {}", command))?;
        }

        Ok(())
    }

    fn open_url(&self, url: &str) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            process::Command::new("explorer")
                .arg(url)
                .spawn()
                .with_context(|| format!("Failed to open URL: {}", url))?;
        }

        #[cfg(target_os = "macos")]
        {
            process::Command::new("open")
                .arg(url)
                .spawn()
                .with_context(|| format!("Failed to open URL: {}", url))?;
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            process::Command::new("xdg-open")
                .arg(url)
                .spawn()
                .with_context(|| format!("Failed to open URL: {}", url))?;
        }

        Ok(())
    }

    pub fn is_bang_command(&self, query: &str) -> bool {
        query.trim().starts_with('!')
    }

    pub fn parse_bang_command(&self, query: &str) -> Option<(String, String)> {
        // Extract the bang command and the search term
        let parts: Vec<&str> = query.splitn(2, ' ').collect();

        if parts.len() == 2 && parts[0].starts_with('!') {
            let bang = parts[0].trim();
            let search_term = parts[1].trim();

            if !bang.is_empty() && !search_term.is_empty() {
                return Some((bang.to_string(), search_term.to_string()));
            }
        }

        None
    }
}
