mod app;
mod args;
mod components;
mod errors;
mod logging;
mod pem;
mod pfx;

fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Args::parse();
    debug!("Args: {:#?}", args);

    let state = AppState::try_from(args)?;
    App::new(250)?.handle_panics().states(state).sets(TheApp).run()?;

    Ok(())
}

use app::{AppState, TheApp};
use clap::Parser as _;
use color_eyre::Result;
use tracing::debug;
use widgetui::*;

use crate::args::Args;
