use anyhow::{Result, Context};
use shared::{config, ipc, logging, models};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

mod hotkey;
mod process;
mod setup;

use hotkey::HotkeyManager;
use process::ProcessManager;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct Bang {
    c: String,  // category
    d: String,  // domain
    r: u32,     // rank
    s: String,  // site name
    sc: String, // subcategory
    t: String,  // trigger (prefix)
    u: String,  // url template
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let proj_dirs = directories::ProjectDirs::from("", "", "orion")
        .context("Failed to get project directories")?;

    let config_dir = proj_dirs.config_dir();
    std::fs::create_dir_all(config_dir)?;

    let log_path = config_dir.join("background.log");

    logging::init(Some(log_path))?;
    logging::info("Background service starting...");

    // Setup configuration
    setup::setup_config().await?;
    logging::info("Configuration setup complete");

    // Load configuration
    let config_path = config_dir.join("config.toml");
    let config_result = config::Config::load(&config_path);

    let config = match config_result {
        Ok(cfg) => {
            logging::info(&format!("Configuration loaded from {}", config_path.display()));
            Arc::new(Mutex::new(cfg))
        },
        Err(e) => {
            logging::error(&format!("Failed to load config: {}. Using default config.", e));
            Arc::new(Mutex::new(config::Config::default()))
        }
    };

    // Get socket path from config
    let socket_path_str = {
        let cfg = config.lock().await;
        cfg.ipc_socket_path.clone()
    };

    let socket_path = PathBuf::from(&socket_path_str);

    // Ensure socket directory exists
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Initialize IPC server
    let ipc_server = ipc::IpcServer::new(socket_path)?;
    let server_addr = ipc_server.address();
    logging::info(&format!("IPC server started at {}", server_addr));

    // Initialize process manager
    let process_manager = Arc::new(ProcessManager::new(&server_addr)?);
    logging::info("Process manager initialized");

    // Initialize hotkey manager
    let mut hotkey_manager = HotkeyManager::new()?;
    logging::info("Hotkey manager initialized");

    // Start IPC server in a separate task
    let ipc_server = Arc::new(ipc_server);
    let ipc_server_clone = ipc_server.clone();
    tokio::spawn(async move {
        if let Err(e) = ipc_server_clone.start_async().await {
            logging::error(&format!("IPC server error: {:?}", e));
        }
    });

    // Set up hotkey listener
    let config_clone = config.clone();
    let process_manager_clone = process_manager.clone();
    hotkey_manager.start_listening(
        &[rdev::Key::ControlLeft, rdev::Key::Alt],
        rdev::Key::Space,
        move || {
            let config = config_clone.clone();
            let process_manager = process_manager_clone.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_hotkey_press(&config, &process_manager).await {
                    logging::error(&format!("Error handling hotkey press: {:?}", e));
                }
            });
        },
    );
    logging::info("Hotkey listener started");

    // Main event loop
    loop {
        match ipc_server.receive_message().await {
            Ok(message) => {
                match message {
                    models::IpcMessage::SearchQuery(query) => {
                        if let Err(e) = handle_search(query, &config, &process_manager).await {
                            logging::error(&format!("Error handling search: {:?}", e));
                        }
                    }
                    models::IpcMessage::Command(cmd) => {
                        if let Err(e) = handle_command(cmd, &config, &process_manager).await {
                            logging::error(&format!("Error handling command: {:?}", e));
                        }
                    }
                    models::IpcMessage::ConfigUpdate => {
                        if let Err(e) = handle_config_update(&config_path, &config).await {
                            logging::error(&format!("Error updating config: {:?}", e));
                        }
                    }
                    models::IpcMessage::Redirect(url) => {
                        if let Err(e) = handle_command(
                            models::Command::new(
                                "Open URL".to_string(),
                                url.clone(),
                                models::Action::OpenUrl(url),
                                vec![],
                            ),
                            &config,
                            &process_manager,
                        ).await {
                            logging::error(&format!("Error handling redirect: {:?}", e));
                        }
                    }
                    _ => {
                        logging::warn("Received unexpected message type");
                    }
                }
            }
            Err(e) => {
                logging::error(&format!("Error receiving message: {:?}", e));
                // Add delay to prevent tight loop on error
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

async fn handle_hotkey_press(
    config: &Arc<Mutex<config::Config>>,
    process_manager: &Arc<ProcessManager>,
) -> Result<()> {
    logging::info("Hotkey pressed, toggling popup UI");

    let config = config.lock().await;
    let _current_profile = config.get_current_profile()?;

    if process_manager.is_popup_running().await {
        logging::info("Popup UI is running, stopping it");
        process_manager.stop_popup().await?;
    } else {
        logging::info("Starting popup UI");
        process_manager.start_popup().await?;

        // Send initial configuration to popup
        let message = models::IpcMessage::ConfigUpdate;
        process_manager.send_message(message).await?;
    }

    Ok(())
}

async fn handle_search(
    query: models::SearchQuery,
    config: &Arc<Mutex<config::Config>>,
    process_manager: &Arc<ProcessManager>,
) -> Result<()> {
    logging::info(&format!("Handling search query: {}", query.text));

    let config = config.lock().await;
    let current_profile = config.get_current_profile()?;

    // Load bangs from file
    let proj_dirs = directories::ProjectDirs::from("", "", "orion")
        .context("Failed to get project directories")?;

    let config_dir = proj_dirs.config_dir();
    let bangs_path = config_dir.join("bangs.json");

    if let Ok(bangs_content) = std::fs::read_to_string(&bangs_path) {
        if let Ok(bangs) = serde_json::from_str::<Vec<models::Bang>>(&bangs_content) {
            // Try to find a bang at the start of the query
            if let Some((prefix, rest)) = query.text.split_once(' ') {
                if let Some(bang) = bangs.iter().find(|b| b.trigger == prefix) {
                    let url = bang.url_template.replace("{{{s}}}", rest);
                    process_manager.send_message(models::IpcMessage::Redirect(url)).await?;
                    return Ok(());
                }
            }

            // Try to find a bang at the end of the query
            if let Some((search, bang)) = query.text.rsplit_once(' ') {
                if let Some(bang) = bangs.iter().find(|b| b.trigger == bang) {
                    let url = bang.url_template.replace("{{{s}}}", search);
                    process_manager.send_message(models::IpcMessage::Redirect(url)).await?;
                    return Ok(());
                }
            }

            // Try to find a bang in the middle of the query
            let words: Vec<&str> = query.text.split(' ').collect();
            for i in 1..words.len()-1 {
                if let Some(bang) = bangs.iter().find(|b| b.trigger == words[i]) {
                    let search = format!("{} {}",
                        words[..i].join(" "),
                        words[i+1..].join(" ")
                    );
                    let url = bang.url_template.replace("{{{s}}}", &search);
                    process_manager.send_message(models::IpcMessage::Redirect(url)).await?;
                    return Ok(());
                }
            }
        }
    }

    // Otherwise, perform normal search
    let mut results = Vec::new();

    // Search in commands
    for cmd in &current_profile.commands {
        // Convert config::Command to models::Command
        let model_cmd = models::Command::new(
            cmd.name.clone(),
            cmd.description.clone(),
            models::Action::OpenUrl(cmd.url.clone()),
            Vec::new()
        );

        if model_cmd.matches_query(&query.text) {
            results.push(models::SearchResult::new(
                cmd.name.clone(),
                Some(cmd.description.clone()),
                models::Action::OpenUrl(cmd.url.clone()),
                1.0
            ));
        }
    }

    // Sort results by score
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    let response = models::SearchResponse {
        results,
        query,
    };

    process_manager.send_message(models::IpcMessage::SearchResponse(response)).await?;
    Ok(())
}

async fn handle_command(
    cmd: models::Command,
    _config: &Arc<Mutex<config::Config>>,
    _process_manager: &Arc<ProcessManager>,
) -> Result<()> {
    logging::info(&format!("Handling command: {}", cmd.name));

    match cmd.action {
        models::Action::OpenFile(path) => {
            logging::info(&format!("Opening file: {:?}", path));
            if cfg!(target_os = "windows") {
                Command::new("explorer").arg(path).spawn()?;
            } else if cfg!(target_os = "macos") {
                Command::new("open").arg(path).spawn()?;
            } else {
                Command::new("xdg-open").arg(path).spawn()?;
            }
        }
        models::Action::ExecuteCommand(command) => {
            logging::info(&format!("Executing command: {}", command));
            if cfg!(target_os = "windows") {
                Command::new("cmd").arg("/C").arg(command).spawn()?;
            } else {
                Command::new("sh").arg("-c").arg(command).spawn()?;
            }
        }
        models::Action::OpenUrl(url) => {
            logging::info(&format!("Opening URL: {}", url));
            if cfg!(target_os = "windows") {
                Command::new("explorer").arg(url).spawn()?;
            } else if cfg!(target_os = "macos") {
                Command::new("open").arg(url).spawn()?;
            } else {
                Command::new("xdg-open").arg(url).spawn()?;
            }
        }
        models::Action::Custom(_) => {
            logging::warn("Custom actions not implemented yet");
        }
    }

    Ok(())
}

async fn handle_config_update(path: &PathBuf, config: &Arc<Mutex<config::Config>>) -> Result<()> {
    logging::info("Updating configuration");

    let new_config = config::Config::load(path)?;
    *config.lock().await = new_config;

    logging::info("Configuration updated successfully");
    Ok(())
}
