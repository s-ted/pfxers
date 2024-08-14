mod args;
mod pem;
mod pfx;

fn main() -> Result<()> {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true).unwrap_or(());

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init()?;

    let args = &Args::parse();
    debug!("Args: {:#?}", args);

    match &args.command {
        args::Commands::PfxInfo(PfxInfoArgs { input, password_file }) => {
            let (key, certs) = pfx::parse_fpx(input, password_file.as_deref())?;

            pfx::show_info(key.as_ref(), &certs);
        }

        args::Commands::PfxToPem(PfxToPemArgs { input, password_file }) => {
            let (key, certs) = pfx::parse_fpx(input, password_file.as_deref())?;

            pfx::show_pem(key.as_ref(), &certs);
        }

        args::Commands::PemInfo(PemInfoArgs { input }) => {
            let (key, certs) = pem::parse_pem(input)?;
            if key.is_some() {
                info!(
                    "this tool is not able to extract private keys information: it only detect \
                     them"
                );
            }
            pfx::show_info(key.as_ref(), &certs);
        }
    }

    Ok(())
}

use anyhow::Result;
use clap::Parser as _;
use log::{debug, info};

use crate::args::{Args, PemInfoArgs, PfxInfoArgs, PfxToPemArgs};
