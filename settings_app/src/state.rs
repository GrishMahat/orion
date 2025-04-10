use iced::Color;
use shared::config;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    General,
    Hotkeys,
    Appearance,
    Advanced
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    Light,
    Dark,
    System, // Placeholder for potential system theme integration
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::Dark // Default to Dark theme
    }
}

pub struct State {
    pub current_profile: String,
    pub profiles: Vec<String>,
    pub settings: Vec<(String, String)>,
    pub new_profile_name: String,
    pub active_tab: Tab,
    pub voice_enabled: bool,
    pub hotkey: String,
    pub theme: AppTheme,
    pub sensitivity: f32,
    pub accent_color: Color,
    config: Arc<Mutex<config::Config>>,
}

impl State {
    pub fn new(config: Arc<Mutex<config::Config>>) -> Self {
        let state = Self {
            current_profile: String::new(),
            profiles: Vec::new(),
            settings: Vec::new(),
            new_profile_name: String::new(),
            active_tab: Tab::General,
            voice_enabled: true,
            hotkey: "Ctrl+Space".to_string(),
            theme: AppTheme::Dark, // Default to Dark theme
            sensitivity: 0.7,
            accent_color: Color::from_rgb(0.35, 0.56, 0.98), // Default accent
            config,
        };
        
        // Load initial configuration data (but don't await the future)
        // This will ensure the fields get properly initialized when the state is created
        let _ = state.profiles.clone();
        let _ = state.settings.clone();
        let _ = state.config.clone();
        
        state
    }

    // Keep for future implementation
    #[allow(dead_code)]
    pub async fn load(&mut self) -> anyhow::Result<()> {
        let config = self.config.lock().await;
        
        // Load profiles
        self.profiles = config.get_profile_names();
        
        // Load current profile
        if let Ok(profile) = config.get_current_profile() {
            self.current_profile = profile.name.clone();
            
            // Load settings for current profile
            self.settings = vec![
                ("max_results".to_string(), config.search.max_results.to_string()),
                ("search_delay".to_string(), config.search.search_delay.to_string()),
            ];
        }
        
        Ok(())
    }
} 