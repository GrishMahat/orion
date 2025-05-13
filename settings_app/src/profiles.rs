use shared::config::{Profile, Config};
use anyhow::Result;

#[allow(dead_code)]
pub fn get_profile_list(profiles: &[Profile]) -> Vec<String> {
    profiles.iter().map(|p| p.name.clone()).collect()
}

pub async fn add_profile(config: &mut Config, name: String) -> Result<()> {
    config.add_profile(name)
}

pub async fn remove_profile(config: &mut Config, name: &str) -> Result<()> {
    config.remove_profile(name)
}

pub async fn select_profile(config: &mut Config, name: &str) -> Result<()> {
    if config.profiles.iter().any(|p| p.name == name) {
        config.current_profile = name.to_string();
        Ok(())
    } else {
        Err(anyhow::anyhow!("Profile '{}' not found", name))
    }
}
