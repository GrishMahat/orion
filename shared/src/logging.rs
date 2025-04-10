use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use log::{Log, Record, Level, Metadata, LevelFilter, info};
use chrono::Local;

pub struct Logger {
    file: Option<Mutex<File>>,
}

impl Logger {
    pub fn new(log_file: Option<PathBuf>) -> anyhow::Result<Self> {
        let file = if let Some(path) = log_file {
            // Create directory if it doesn't exist
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }
            Some(Mutex::new(fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?))
        } else {
            None
        };

        Ok(Logger { file })
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now();
            let message = format!("[{}] {} - {}\n",
                now.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            );

            if let Some(file) = &self.file {
                if let Ok(mut file) = file.lock() {
                    let _ = file.write_all(message.as_bytes());
                }
            }

            println!("{}", message);
        }
    }

    fn flush(&self) {
        if let Some(file) = &self.file {
            if let Ok(mut file) = file.lock() {
                let _ = file.flush();
            }
        }
    }
}

pub fn init(log_file: Option<PathBuf>) -> anyhow::Result<()> {
    let logger = Logger::new(log_file)?;
    log::set_max_level(LevelFilter::Info);
    log::set_logger(Box::leak(Box::new(logger)))?;
    info!("Logger initialized successfully");
    Ok(())
}

pub fn error(msg: &str) {
    log::error!("{}", msg);
}

pub fn warn(msg: &str) {
    log::warn!("{}", msg);
}

pub fn info(msg: &str) {
    log::info!("{}", msg);
}

pub fn debug(msg: &str) {
    log::debug!("{}", msg);
}

pub fn trace(msg: &str) {
    log::trace!("{}", msg);
}
