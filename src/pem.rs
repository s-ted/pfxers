pub(crate) fn parse_pem(input: &str) -> Result<(Option<PrivateKeyChain>, Vec<Certificate>)> {
    let pem_content = std::fs::read(input)?;

    let pem = parse_many(pem_content)?;

    let mut key = None;
    let mut certs = Vec::new();

    for p in pem {
        match p.tag() {
            "PRIVATE KEY" => {
                key = Some(PrivateKeyChain::new(
                    p.contents(),
                    "dummy-local-key-id".as_bytes(),
                    Vec::new(),
                ))
            }
            "CERTIFICATE" => certs.push(Certificate::from_der(p.contents())?),

            _ => {
                warn!("Unknown PEM tag: {}", p.tag());
            }
        }
    }

    Ok((key, certs))
}

use anyhow::Result;
use log::warn;
use p12_keystore::{Certificate, PrivateKeyChain};
use pem::parse_many;
