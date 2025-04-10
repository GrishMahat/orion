use anyhow::Result;
use iced::{Application, Color, Command, Element, Theme};
use shared::config;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state::{AppTheme, State, Tab};
use crate::ui::view;

pub struct App {
    state: State,
    config: Arc<Mutex<config::Config>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadConfig,
    SaveConfig,
    UpdateProfile(String),
    AddProfile(String),
    RemoveProfile(String),
    UpdateSetting(String, String),
    Error(()),
    NewProfileNameChanged(String),
    TabSelected(Tab),
    ToggleVoice(bool),
    UpdateHotkey(String),
    UpdateSensitivity(f32),
    SetTheme(AppTheme),
    SetAccentColor(Color),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config_path = directories::ProjectDirs::from("com", "orion", "config")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("config.toml"));

        let config = Arc::new(Mutex::new(config::Config::load(&config_path).unwrap_or_default()));
        let state = State::new(config.clone());

        (
            Self { state, config },
            Command::perform(load_config(config_path), |result| {
                match result {
                    Ok(_) => Message::LoadConfig,
                    Err(_) => Message::Error(()),
                }
            }),
        )
    }

    fn title(&self) -> String {
        "Orion Settings".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadConfig => {
                // Config is already loaded in new()
                Command::none()
            }
            Message::SaveConfig => {
                let config = self.config.clone();
                Command::perform(save_config(config), |result| {
                    match result {
                        Ok(_) => Message::LoadConfig,
                        _ => Message::Error(()),
                    }
                })
            }
            Message::UpdateProfile(name) => {
                self.state.current_profile = name;
                Command::none()
            }
            Message::AddProfile(name) => {
                let config = self.config.clone();
                Command::perform(add_profile(config, name), |result| {
                    match result {
                        Ok(_) => Message::LoadConfig,
                        Err(_) => Message::Error(()),
                    }
                })
            }
            Message::RemoveProfile(name) => {
                let config = self.config.clone();
                Command::perform(remove_profile(config, name), |result| {
                    match result {
                        Ok(_) => Message::LoadConfig,
                        Err(_) => Message::Error(()),
                    }
                })
            }
            Message::UpdateSetting(key, value) => {
                let config = self.config.clone();
                Command::perform(update_setting(config, key, value), |result| {
                    match result {
                        Ok(_) => Message::LoadConfig,
                        Err(_) => Message::Error(()),
                    }
                })
            }
            Message::Error(_) => Command::none(),
            Message::NewProfileNameChanged(name) => {
                self.state.new_profile_name = name;
                Command::none()
            }
            Message::TabSelected(tab) => {
                self.state.active_tab = tab;
                Command::none()
            }
            Message::ToggleVoice(enabled) => {
                self.state.voice_enabled = enabled;
                Command::none()
            }
            Message::UpdateHotkey(hotkey) => {
                self.state.hotkey = hotkey;
                Command::none()
            }
            Message::UpdateSensitivity(sensitivity) => {
                self.state.sensitivity = sensitivity;
                Command::none()
            }
            Message::SetTheme(theme) => {
                self.state.theme = theme;
                Command::none()
            }
            Message::SetAccentColor(color) => {
                self.state.accent_color = color;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        view(&self.state)
    }

    fn theme(&self) -> Theme {
        // TODO: Implement theme switching logic based on self.state.theme
        // For now, default to dark
        Theme::Dark
    }
}

async fn load_config(path: PathBuf) -> Result<()> {
    let _config = config::Config::load(&path)?;
    Ok(())
}

async fn save_config(config: Arc<Mutex<config::Config>>) -> Result<()> {
    let config = config.lock().await;
    let path = directories::ProjectDirs::from("com", "orion", "config")
        .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("config.toml"));
    
    config.save(&path)?;
    Ok(())
}

async fn add_profile(config: Arc<Mutex<config::Config>>, name: String) -> Result<()> {
    let mut config = config.lock().await;
    config.add_profile(name)?;
    Ok(())
}

async fn remove_profile(config: Arc<Mutex<config::Config>>, name: String) -> Result<()> {
    let mut config = config.lock().await;
    config.remove_profile(&name)?;
    Ok(())
}

async fn update_setting(config: Arc<Mutex<config::Config>>, key: String, value: String) -> Result<()> {
    let mut config = config.lock().await;
    config.update_settings(vec![(key, value)])?;
    Ok(())
} 