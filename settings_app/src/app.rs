use anyhow::Result;
use iced::{Application, Command, Element, executor, Theme, Settings, window};
use shared::{config, ipc, logging};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use iced::widget::text_editor::Action;
use iced::{Color};

use crate::state::{State, Tab, AppTheme};
use crate::ui::{self, TabUI};
use crate::profiles::ProfileManager;

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
    profile_manager: Option<ProfileManager>,
    config_path: PathBuf,
    ipc_client: Option<Arc<Mutex<ipc::IpcClient>>>,
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
            profile_manager: None,
            config_path,
            ipc_client: None,
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
                self.state.current_profile = profile;
            }
            AppMessage::AddProfile => {
                if !self.state.new_profile_name.trim().is_empty()
                   && !self.state.profiles.contains(&self.state.new_profile_name) {
                    self.state.profiles.push(self.state.new_profile_name.clone());
                    self.state.new_profile_name.clear();
                }
            }
            AppMessage::UpdateNewProfileName(name) => {
                self.state.new_profile_name = name;
            }
            AppMessage::DeleteProfile(profile) => {
                if profile != "Default" && self.state.profiles.contains(&profile) {
                    self.state.profiles.retain(|p| p != &profile);
                    if self.state.current_profile == profile {
                        self.state.current_profile = "Default".to_string();
                    }
                }
            }
            AppMessage::SaveSettings => {
                println!("Settings saved!");
            }
            AppMessage::ResetSettings => {
                self.state = State::new();
            }
            AppMessage::LoadConfig(config) => {
                // Update State with loaded config
                self.state = State::new(config);
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
