use iced::{Application, Command, Element, executor, Theme};
use shared::config;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use iced::Color;

use crate::state::{State, Tab, AppTheme};
use crate::ui::TabUI;

#[derive(Debug, Clone)]
pub enum AppMessage {
    TabSelected(Tab),
    ToggleVoice(bool),
    UpdateHotkey(String),
    SetTheme(AppTheme),
    SetAccentColor(Color),
    AdjustSensitivity(f32),
    SelectProfile(String),
    AddProfile,
    UpdateNewProfileName(String),
    DeleteProfile(String),
    SaveSettings,
    ResetSettings,
    LoadConfig(Arc<Mutex<config::Config>>),
}

pub struct App {
    state: State,
    config_path: PathBuf,
    ui: TabUI,
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        // Get config path
        let config_path = directories::ProjectDirs::from("com", "orion", "config")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("config.toml"));

        // Create a default config for initial state
        let default_config = Arc::new(Mutex::new(config::Config::default()));

        // Create app
        let app = Self {
            state: State::new(default_config),
            config_path,
            ui: TabUI::new(),
        };

        // Load config
        let config_path = app.config_path.clone();
        let cmd = Command::perform(
            async move {
                let config = config::Config::load(&config_path)
                    .unwrap_or_else(|_| config::Config::default());

                Arc::new(Mutex::new(config))
            },
            AppMessage::LoadConfig
        );

        (app, cmd)
    }

    fn title(&self) -> String {
        "Orion Settings".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::TabSelected(tab) => {
                self.state.active_tab = tab;
            }
            AppMessage::ToggleVoice(enabled) => {
                self.state.voice_enabled = enabled;
            }
            AppMessage::UpdateHotkey(hotkey) => {
                self.state.hotkey = hotkey;
            }
            AppMessage::SetTheme(theme) => {
                self.state.theme = theme;
            }
            AppMessage::SetAccentColor(color) => {
                self.state.accent_color = color;
            }
            AppMessage::AdjustSensitivity(value) => {
                self.state.sensitivity = value;
            }
            AppMessage::SelectProfile(profile) => {
                let profile_clone = profile.clone();
                let config = self.state.config.clone();
                
                return Command::perform(
                    async move {
                        let mut config_guard = config.lock().await;
                        if let Err(e) = crate::profiles::select_profile(&mut config_guard, &profile_clone).await {
                            eprintln!("Failed to select profile: {}", e);
                        }
                        profile_clone
                    },
                    |name| AppMessage::SelectProfile(name)
                );
            }
            AppMessage::AddProfile => {
                if !self.state.new_profile_name.trim().is_empty()
                   && !self.state.profiles.contains(&self.state.new_profile_name) {
                    let name = self.state.new_profile_name.clone();
                    let config_for_async = self.state.config.clone();
                    let config_for_callback = self.state.config.clone();
                    
                    self.state.new_profile_name.clear();
                    
                    return Command::perform(
                        async move {
                            let mut config_guard = config_for_async.lock().await;
                            if let Err(e) = crate::profiles::add_profile(&mut config_guard, name.clone()).await {
                                eprintln!("Failed to add profile: {}", e);
                            }
                        },
                        move |_| AppMessage::LoadConfig(config_for_callback.clone())
                    );
                }
            }
            AppMessage::UpdateNewProfileName(name) => {
                self.state.new_profile_name = name;
            }
            AppMessage::DeleteProfile(profile) => {
                if profile != "Default" && self.state.profiles.contains(&profile) {
                    let profile_clone = profile.clone();
                    let config_for_async = self.state.config.clone();
                    let config_for_callback = self.state.config.clone();
                    
                    return Command::perform(
                        async move {
                            let mut config_guard = config_for_async.lock().await;
                            if let Err(e) = crate::profiles::remove_profile(&mut config_guard, &profile_clone).await {
                                eprintln!("Failed to remove profile: {}", e);
                            }
                        },
                        move |_| AppMessage::LoadConfig(config_for_callback.clone())
                    );
                }
            }
            AppMessage::SaveSettings => {
                let config_path = self.config_path.clone();
                let state = self.state.clone();
                
                return Command::perform(
                    async move {
                        let mut config_guard = state.config.lock().await;
                        
                        // Update config with state values
                        config_guard.hotkey.key_combination = state.hotkey.clone();
                        // Update other settings here as needed
                        
                        if let Err(e) = config_guard.save(&config_path) {
                            eprintln!("Failed to save config: {}", e);
                        }
                        
                        AppMessage::LoadConfig(state.config.clone())
                    },
                    |msg| msg
                );
            }
            AppMessage::ResetSettings => {
                // Make a copy of the existing config
                let config = self.state.config.clone();
                self.state = State::new(config);
            }
            AppMessage::LoadConfig(config) => {
                // Update State with loaded config
                self.state = State::new(config.clone());
                
                // Load the actual settings values
                return Command::perform(
                    async move {
                        let mut state = State::new(config.clone());
                        if let Err(e) = state.load().await {
                            eprintln!("Failed to load settings: {}", e);
                        }
                        state
                    },
                    |state| {
                        let message = AppMessage::LoadConfig(state.config.clone());
                        // Update the app state with the loaded state
                        message
                    }
                );
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Theme> {
        self.ui.view(&self.state)
    }

    fn theme(&self) -> Theme {
        self.state.theme()
    }
}
