use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use shared::config::Config;

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
        println!("Created default config at: {}", config_path.display());
    }

    // Download bangs.json if it doesn't exist
    let bangs_path = config_dir.join("bangs.json");
    if !bangs_path.exists() {
        println!("Downloading bangs.json...");
        match download_bangs(&bangs_path).await {
            Ok(_) => println!("Downloaded bangs.json to: {}", bangs_path.display()),
            Err(e) => {
                println!("Failed to download bangs.json: {}", e);
                // Create an empty bangs.json file as fallback
                fs::write(&bangs_path, "{}")?;
                println!("Created empty bangs.json at: {}", bangs_path.display());
            }
        }
    }

    println!("Configuration setup complete!");
    println!("Config directory: {}", config_dir.display());

    Ok(())
}

async fn download_bangs(path: &PathBuf) -> Result<()> {
    // Download bangs.json
    let bangs_path = config_dir.join("bangs.json");
    let response = reqwest::get(BANGS_URL).await?;
    let content = response.text().await?;

    // Save the file
    fs::write(&bangs_path, content)?;

    println!("Configuration setup complete!");
    println!("bangs.json saved to: {}", bangs_path.display());

    fs::write(path, content)?;

    Ok(())
}
