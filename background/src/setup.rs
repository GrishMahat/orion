use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use shared::{config::Config, logging};

const BANGS_URL: &str = "https://gist.githubusercontent.com/GrishMahat/9500aa4a883650d21bc428abf1adb0d7/raw/723868e88db267fada918f8143e55cca36d10e97/bangs.json";

pub async fn setup_config() -> Result<()> {
    // Get the config directory
    let proj_dirs = ProjectDirs::from("", "", "orion")
        .context("Failed to get project directories")?;

    // Create config directory if it doesn't exist
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;

    // Create config.toml if it doesn't exist
    let config_path = config_dir.join("config.toml");
    if !config_path.exists() {
        let mut default_config = Config::default();

        // Set up socket path in the config directory
        let socket_path = config_dir.join("orion.sock");
        default_config.ipc_socket_path = socket_path.to_string_lossy().to_string();

        // Set up log file path in the config directory
        let log_path = config_dir.join("orion.log");
        default_config.log_file = Some(log_path.to_string_lossy().to_string());

        // Save the default config
        default_config.save(&config_path)?;
        logging::info(&format!("Created default config at: {}", config_path.display()));
    }

    // Handle bangs.json setup
    let bangs_path = config_dir.join("bangs.json");
    if !bangs_path.exists() {
        logging::info("Setting up bangs.json file...");
        
        // First, try to use local version in the project directory
        let local_bangs_path = PathBuf::from("bangs.json");
        
        if local_bangs_path.exists() {
            // Copy the local file to config directory
            logging::info(&format!("Copying local bangs.json to: {}", bangs_path.display()));
            fs::copy(&local_bangs_path, &bangs_path)?;
        } else {
            // Download from remote if local doesn't exist
            logging::info("Downloading bangs.json from remote source...");
            match download_bangs(&bangs_path).await {
                Ok(_) => logging::info(&format!("Downloaded bangs.json to: {}", bangs_path.display())),
                Err(e) => {
                    logging::error(&format!("Failed to download bangs.json: {}", e));
                    // Create an empty bangs.json file as fallback
                    fs::write(&bangs_path, "[]")?;
                    logging::warn(&format!("Created empty bangs.json at: {}", bangs_path.display()));
                }
            }
        }
    }

    logging::info("Configuration setup complete!");
    logging::info(&format!("Config directory: {}", config_dir.display()));

    Ok(())
}

async fn download_bangs(path: &PathBuf) -> Result<()> {
    // Download bangs.json
    let response = reqwest::get(BANGS_URL).await?;
    let content = response.text().await?;

    // Save the file
    fs::write(path, content)?;

    logging::info(&format!("bangs.json downloaded and saved to: {}", path.display()));

    Ok(())
}
