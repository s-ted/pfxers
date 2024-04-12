#[derive(Parser, Debug, Clone)]
#[command(about = "PFX/PEM tools")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Display information about a PFX/PKCS12 file
    PfxInfo(PfxInfoArgs),

    /// Extract PEM (key and certificates) from a PFX/PKCS12 file
    PfxToPem(PfxToPemArgs),

    /// Display information about a PEM file
    PemInfo(PemInfoArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct PfxInfoArgs {
    /// The PFX/PKCS12 file to inspect
    #[arg(long = "input")]
    pub input: String,

    /// The file containing the password of the PFX/PKCS12 file
    /// will be prompted if not specified
    #[arg(long = "password-file")]
    pub password_file: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct PfxToPemArgs {
    /// The PFX/PKCS12 file to inspect
    #[arg(long = "input")]
    pub input: String,

    /// The file containing the password of the PFX/PKCS12 file
    /// will be prompted if not specified
    #[arg(long = "password-file")]
    pub password_file: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct PemInfoArgs {
    /// The PEM file to inspect
    #[arg(long = "input")]
    pub input: String,
}

use clap::{Parser, Subcommand};
