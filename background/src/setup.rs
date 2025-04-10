use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use directories::ProjectDirs;

const BANGS_URL: &str = "https://gist.githubusercontent.com/GrishMahat/9500aa4a883650d21bc428abf1adb0d7/raw/723868e88db267fada918f8143e55cca36d10e97/bangs.json";

pub async fn setup_config() -> Result<()> {
    // Get the config directory
    let proj_dirs = ProjectDirs::from("com", "orion", "config")
        .context("Failed to get project directories")?;
    
    // Create config directory if it doesn't exist
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;
    
    // Download bangs.json
    let bangs_path = config_dir.join("bangs.json");
    let response = reqwest::get(BANGS_URL).await?;
    let content = response.text().await?;
    
    // Save the file
    fs::write(&bangs_path, content)?;
    
    println!("Configuration setup complete!");
    println!("bangs.json saved to: {}", bangs_path.display());
    
    Ok(())
} 