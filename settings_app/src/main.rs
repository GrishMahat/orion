use anyhow::Result;
use iced::{Settings, Application};

mod app;
mod ui;
mod state;
mod profiles;

fn main() -> Result<()> {
    app::App::run(Settings::default())?;
    Ok(())
}
