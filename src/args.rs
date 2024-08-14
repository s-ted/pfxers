#[derive(Parser, Debug, Clone)]
#[command(about, author, version)]
pub struct Args {
    /// The PFX/PKCS12/pem file to inspect
    pub input: String,

    /// The file containing the password of the PFX/PKCS12 file
    #[arg(long = "password-file")]
    pub password_file: Option<String>,

    /// The  password of the PFX/PKCS12 file
    /// You should prefer the use of --password-file
    /// or use the PFX_PASSWORD environment variable
    #[arg(long = "password", env = "PFX_PASSWORD")]
    pub password: Option<Hide<String>>,
}

use clap::Parser;
use hide::Hide;
