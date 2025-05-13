use iced::Color;
use shared::config;
use std::sync::Arc;
use tokio::sync::Mutex;
use iced::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    General,
    Hotkeys,
    Appearance,
    Advanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppTheme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone)]
pub struct State {
    pub config: Arc<Mutex<config::Config>>,
    pub active_tab: Tab,
    pub profiles: Vec<String>,
    pub current_profile: String,
    pub new_profile_name: String,
    pub voice_enabled: bool,
    pub hotkey: String,
    pub theme: AppTheme,
    pub sensitivity: f32,
    pub accent_color: Color,
    pub settings: Vec<(String, String)>,
}

impl State {
    pub fn new(config: Arc<Mutex<config::Config>>) -> Self {
        // Create a default state
        Self {
            config: config.clone(),
            active_tab: Tab::General,
            profiles: vec!["Default".to_string()],
            current_profile: "Default".to_string(),
            new_profile_name: String::new(),
            voice_enabled: true,
            hotkey: "Alt+Space".to_string(),
            theme: AppTheme::System,
            sensitivity: 0.7,
            accent_color: Color::from_rgb(0.4, 0.4, 0.9),
            settings: Vec::new(),
        }
    }

    pub fn theme(&self) -> Theme {
        match self.theme {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
            AppTheme::System => Theme::Dark, // Default to dark theme without dark_light
        }
    }

    pub async fn load(&mut self) -> anyhow::Result<()> {
        let config = self.config.lock().await;

        // Load profiles
        self.profiles = config.get_profile_names();
        if self.profiles.is_empty() {
            self.profiles = vec!["Default".to_string()];
        }

        // Load current profile from config
        self.current_profile = config.current_profile.clone();
        
        // Make sure current_profile is valid
        if !self.profiles.contains(&self.current_profile) {
            self.current_profile = self.profiles.first().unwrap_or(&"Default".to_string()).clone();
        }
            
        // Load hotkey settings
        self.hotkey = config.hotkey.key_combination.clone();
            
        // Load settings for current profile
        self.settings = vec![
            ("max_results".to_string(), config.search.max_results.to_string()),
            ("search_delay".to_string(), config.search.search_delay.to_string()),
        ];

        Ok(())
    }
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tab::General => write!(f, "General"),
            Tab::Hotkeys => write!(f, "Hotkeys"),
            Tab::Appearance => write!(f, "Appearance"),
            Tab::Advanced => write!(f, "Advanced"),
        }
    }
}

impl std::fmt::Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppTheme::System => write!(f, "System"),
            AppTheme::Light => write!(f, "Light"),
            AppTheme::Dark => write!(f, "Dark"),
        }
    }
}
