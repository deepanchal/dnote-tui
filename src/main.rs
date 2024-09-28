pub mod action;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod dnote;
pub mod errors;
pub mod logging;
pub mod state;
pub mod tui;

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}
