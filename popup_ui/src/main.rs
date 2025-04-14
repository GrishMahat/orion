use anyhow::Result;
use iced::{
    Application, Command, Element, executor, Theme, keyboard, event, window,
    Event, Subscription, Settings,
};
use iced::keyboard::{Key, key};
use shared::{ipc, models, logging};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

mod ui;
mod commands;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let log_path = directories::ProjectDirs::from("com", "orion", "logs")
        .map(|proj_dirs| proj_dirs.data_dir().join("popup.log"))
        .unwrap_or_else(|| std::path::PathBuf::from("popup.log"));

    logging::init(Some(log_path))?;
    logging::info("Popup UI starting...");

    // Get IPC server address from command line arguments or use default
    let server_addr = env::args().nth(1).unwrap_or_else(|| "background.sock".to_string());
    logging::info(&format!("Using IPC server at: {}", server_addr));

    // Start the Iced application
    OrionApp::run(Settings::with_flags(OrionSettings {
        server_addr,
        flags: (),
    }))
    .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))
}

struct OrionSettings {
    server_addr: String,
    flags: (),
}

struct OrionApp {
    state: AppState,
    ipc_client: Arc<Mutex<ipc::IpcClient>>,
}

#[derive(Debug, Clone)]
enum AppMessage {
    UiMessage(ui::Message),
    KeyPressed(Key),
    WindowEvent(window::Event),
    SearchCompleted(Vec<models::SearchResult>),
    ExecuteCommand(models::Command),
    CloseRequested,
    IpcMessage(models::IpcMessage),
}

impl Application for OrionApp {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = OrionSettings;

    fn new(settings: Self::Flags) -> (Self, Command<Self::Message>) {
        let ipc_client = match ipc::IpcClient::new(&settings.server_addr) {
            Ok(client) => Arc::new(Mutex::new(client)),
            Err(e) => {
                logging::error(&format!("Failed to connect to IPC server: {}", e));
                std::process::exit(1);
            }
        };

        let app = Self {
            state: AppState::new(),
            ipc_client,
        };

        // Send initial query to get default results
        let cmd = Command::perform(
            async move { models::SearchQuery { text: String::new(), max_results: 10 } },
            |query| {
                AppMessage::ExecuteCommand(models::Command::new(
                    "Initial Query".to_string(),
                    "".to_string(),
                    models::Action::Custom(query.text),
                    vec![]
                ))
            }
        );

        (app, cmd)
    }

    fn title(&self) -> String {
        "Orion".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::UiMessage(ui_msg) => {
                let should_search = self.state.update_search_ui(ui_msg);

                if should_search {
                    if let Some(query) = self.state.get_search_query() {
                        let ipc_client = self.ipc_client.clone();
                        return Command::perform(
                            async move {
                                let mut client = ipc_client.lock().await;
                                let message = models::IpcMessage::SearchQuery(query);
                                client.send_message_async(&message).await?;

                                // Wait for response
                                let response = client.receive_message_async().await?;
                                Ok::<_, anyhow::Error>(response)
                            },
                            |result| match result {
                                Ok(models::IpcMessage::SearchResponse(response)) => {
                                    AppMessage::SearchCompleted(response.results)
                                }
                                Ok(msg) => AppMessage::IpcMessage(msg),
                                Err(e) => {
                                    logging::error(&format!("IPC error: {}", e));
                                    AppMessage::SearchCompleted(vec![])
                                }
                            }
                        );
                    }
                }

                Command::none()
            }
            AppMessage::KeyPressed(key) => {
                match key {
                    Key::Named(key::Named::Escape) => {
                        return Command::perform(async {}, |_| AppMessage::CloseRequested);
                    }
                    Key::Named(key::Named::ArrowUp) |
                    Key::Named(key::Named::ArrowDown) |
                    Key::Named(key::Named::Enter) => {
                        if let Some(cmd) = self.state.handle_keypress(key) {
                            return Command::perform(async { cmd }, AppMessage::ExecuteCommand);
                        }
                    }
                    _ => {}
                }

                Command::none()
            }
            AppMessage::WindowEvent(event) => {
                if let window::Event::CloseRequested = event {
                    return Command::perform(async {}, |_| AppMessage::CloseRequested);
                }

                Command::none()
            }
            AppMessage::SearchCompleted(results) => {
                self.state.process_search_results(results);
                Command::none()
            }
            AppMessage::ExecuteCommand(cmd) => {
                let ipc_client = self.ipc_client.clone();

                Command::perform(
                    async move {
                        let mut client = ipc_client.lock().await;
                        let message = models::IpcMessage::Command(cmd);
                        client.send_message_async(&message).await?;

                        // Don't wait for response for commands
                        Ok::<_, anyhow::Error>(())
                    },
                    |result| {
                        if let Err(e) = result {
                            logging::error(&format!("Error executing command: {}", e));
                        }
                        AppMessage::CloseRequested
                    }
                )
            }
            AppMessage::CloseRequested => {
                logging::info("Close requested, exiting...");
                window::close(window::Id::MAIN)
            }
            AppMessage::IpcMessage(msg) => {
                self.state.process_ipc_message(msg);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message, Theme> {
        self.state.view().map(AppMessage::UiMessage)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            event::listen().map(|event| {
                match event {
                    Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                        AppMessage::KeyPressed(key)
                    }
                    Event::Window(_id, window_event) => AppMessage::WindowEvent(window_event),
                    _ => AppMessage::UiMessage(ui::Message::CloseRequested),
                }
            }),
        ])
    }
}
