use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hotkey: HotkeyConfig,
    pub search: SearchConfig,
    pub profiles: Vec<Profile>,
    pub current_profile: String,
    pub log_level: String,
    pub log_file: Option<String>,
    pub ipc_socket_path: String,
    pub command_prefixes: Vec<CommandPrefix>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub key_combination: String,
    pub modifiers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchConfig {
    pub max_results: usize,
    pub search_delay: u64,
}

impl SearchConfig {
    pub fn validate(&self) -> Result<()> {
        if self.max_results < 1 || self.max_results > 100 {
            return Err(anyhow::anyhow!("max_results must be between 1 and 100"));
        }
        if self.search_delay < 100 || self.search_delay > 5000 {
            return Err(anyhow::anyhow!("search_delay must be between 100 and 5000"));
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub commands: Vec<Command>,
}

impl Profile {
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(anyhow::anyhow!("Profile name cannot be empty"));
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileSettings {
    pub theme: String,
    pub shortcuts: Vec<Shortcut>,
    pub search_paths: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shortcut {
    pub name: String,
    pub command: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandPrefix {
    pub prefix: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub name: String,
    pub url: String,
    pub description: String,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.current_profile.is_empty() {
            return Err(anyhow::anyhow!("Current profile cannot be empty"));
        }
        
        self.search.validate()?;
        
        for profile in &self.profiles {
            profile.validate()?;
        }
        
        Ok(())
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file at {:?}", path))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file at {:?}", path))?;
        config.validate()?;
        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        self.validate()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_current_profile(&self) -> Result<&Profile> {
        self.profiles
            .iter()
            .find(|p| p.name == self.current_profile)
            .with_context(|| format!("Current profile '{}' not found", self.current_profile))
    }

    pub fn get_profile_names(&self) -> Vec<String> {
        self.profiles.iter().map(|p| p.name.clone()).collect()
    }

    pub fn add_profile(&mut self, name: String) -> Result<()> {
        if self.profiles.iter().any(|p| p.name == name) {
            return Err(anyhow::anyhow!("Profile '{}' already exists", name));
        }
        self.profiles.push(Profile {
            name,
            commands: Vec::new(),
        });
        Ok(())
    }

    pub fn remove_profile(&mut self, name: &str) -> Result<()> {
        if name == self.current_profile {
            return Err(anyhow::anyhow!("Cannot remove current profile"));
        }
        self.profiles.retain(|p| p.name != name);
        Ok(())
    }

    pub fn update_settings(&mut self, settings: Vec<(String, String)>) -> Result<()> {
        for (key, value) in settings {
            match key.as_str() {
                "max_results" => {
                    self.search.max_results = value.parse()?;
                }
                "search_delay" => {
                    self.search.search_delay = value.parse()?;
                }
                _ => return Err(anyhow::anyhow!("Unknown setting: {}", key)),
            }
        }
        self.validate()?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hotkey: HotkeyConfig {
                key_combination: "Alt+Space".to_string(),
                modifiers: vec!["Alt".to_string()],
            },
            search: SearchConfig {
                max_results: 10,
                search_delay: 200,
            },
            profiles: vec![
                Profile {
                    name: "Default".to_string(),
                    commands: Vec::new(),
                }
            ],
            current_profile: "Default".to_string(),
            log_level: "info".to_string(),
            log_file: None,
            ipc_socket_path: "orion.sock".to_string(),
            command_prefixes: Vec::new(),
        }
    }
} 