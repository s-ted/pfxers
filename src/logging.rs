pub(crate) fn init() -> Result<()> {
    let env_filter =
        EnvFilter::builder().with_default_directive(tracing::Level::INFO.into()).try_from_env()?;

    tracing_subscriber::registry().with(env_filter).with(ErrorLayer::default()).try_init()?;
    Ok(())
}

use color_eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
