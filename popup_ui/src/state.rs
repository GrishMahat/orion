use anyhow::Result;
use std::time::{Duration, Instant};
use shared::models::{SearchQuery, SearchResult, IpcMessage, Command};
use crate::ui::SearchUI;
use crate::commands::CommandExecutor;
use iced::keyboard::Key;

const SEARCH_DELAY: Duration = Duration::from_millis(200);

pub struct AppState {
    search_ui: SearchUI,
    command_executor: CommandExecutor,
    last_search_time: Option<Instant>,
    current_query: Option<SearchQuery>,
    is_searching: bool,
    search_results: Vec<SearchResult>,
    command_history: Vec<String>,
    max_history: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            search_ui: SearchUI::new(),
            command_executor: CommandExecutor::new(),
            last_search_time: None,
            current_query: None,
            is_searching: false,
            search_results: Vec::new(),
            command_history: Vec::new(),
            max_history: 100,
        }
    }

    pub fn update_search_ui(&mut self, message: crate::ui::Message) -> bool {
        let should_search = self.search_ui.update(message);

        if should_search {
            self.queue_search();
        }

        should_search
    }

    pub fn view(&self) -> iced::Element<'_, crate::ui::Message, iced::Theme> {
        self.search_ui.view()
    }

    pub fn should_perform_search(&self) -> bool {
        if let Some(last_time) = self.last_search_time {
            if self.is_searching {
                Instant::now().duration_since(last_time) >= SEARCH_DELAY
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn queue_search(&mut self) {
        self.last_search_time = Some(Instant::now());
        self.is_searching = true;
    }

    pub fn get_search_query(&self) -> Option<SearchQuery> {
        if self.is_searching {
            Some(self.search_ui.get_search_query())
        } else {
            None
        }
    }

    pub fn process_search_results(&mut self, results: Vec<SearchResult>) {
        self.is_searching = false;
        self.search_results = results.clone();
        self.search_ui.set_results(results);
    }

    pub fn handle_keypress(&mut self, key: Key) -> Option<Command> {
        match key {
            Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                self.search_ui.select_next();
                None
            }
            Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                self.search_ui.select_previous();
                None
            }
            Key::Named(iced::keyboard::key::Named::Enter) => {
                // Get the selected result and convert to a command
                if let Some(result) = self.search_ui.get_selected_result() {
                    // Add to command history
                    if self.command_history.len() >= self.max_history {
                        self.command_history.remove(0);
                    }
                    self.command_history.push(result.title.clone());

                    // Create a command from the result
                    Some(Command::new(
                        result.title.clone(),
                        result.description.clone().unwrap_or_default(),
                        result.action.clone(),
                        Vec::new(),
                    ))
                } else {
                    None
                }
            }
            Key::Named(iced::keyboard::key::Named::Escape) => {
                // Signal to close the UI
                None
            }
            _ => None,
        }
    }

    pub fn execute_command(&self, command: &Command) -> Result<()> {
        self.command_executor.execute(command)
    }

    pub fn process_ipc_message(&mut self, message: IpcMessage) {
        match message {
            IpcMessage::SearchResponse(response) => {
                self.process_search_results(response.results);
            }
            IpcMessage::Redirect(url) => {
                let cmd = Command::new(
                    "Open URL".to_string(),
                    url.clone(),
                    shared::models::Action::OpenUrl(url),
                    Vec::new(),
                );

                if let Err(err) = self.execute_command(&cmd) {
                    eprintln!("Error executing redirect: {:?}", err);
                }
            }
            _ => {
                // Handle other IPC messages as needed
            }
        }
    }

    pub fn get_command_history(&self) -> &[String] {
        &self.command_history
    }
}
