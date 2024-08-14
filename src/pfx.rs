pub(crate) fn parse_pfx(
    pfx_file_path: &str,
    password_file_path: Option<&str>,
    password: Option<Hide<String>>,
) -> Result<(Option<PrivateKeyChain>, Vec<Certificate>)> {
    let pfx_content = std::fs::read(pfx_file_path)?;
    let password = if let Some(password) = password {
        password
    } else {
        if let Some(password_file_path) = password_file_path {
            std::fs::read_to_string(password_file_path)
                .map_err(PfxError::UnableToReadPasswordFile)?
        } else {
            String::new()
        }
        .into()
    };

    let pfx =
        KeyStore::from_pkcs12(&pfx_content, password.as_str()).map_err(PfxError::ParseError)?;

    let keys = pfx
        .entries()
        .filter_map(|(_, e)| match e {
            KeyStoreEntry::PrivateKeyChain(x) => Some(x.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();

    let keys_certificates = keys.iter().flat_map(|key| key.chain());

    let certs = pfx.entries().filter_map(|(_, e)| match e {
        KeyStoreEntry::Certificate(x) => Some(x),
        _ => None,
    });

    Ok((keys.first().cloned(), keys_certificates.chain(certs).cloned().collect()))
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum PfxError {
    #[error(
        "Unable to parse PFX file, are you sure you gave the right password using --password or \
         --password-file?"
    )]
    ParseError(#[from] p12_keystore::error::Error),

    #[error("Unable to read password file given in --password-file")]
    UnableToReadPasswordFile(#[from] std::io::Error),
}

use color_eyre::Result;
use hide::Hide;
use p12_keystore::{Certificate, KeyStore, KeyStoreEntry, PrivateKeyChain};
