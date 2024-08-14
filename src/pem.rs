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

pub(crate) struct Base64Pem {
    content: String,
}

impl Display for Base64Pem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.content)
    }
}

impl From<&Certificate> for Base64Pem {
    fn from(certificate: &Certificate) -> Self {
        let pem = Pem::new("CERTIFICATE", certificate.as_der());
        let content = encode(&pem);
        Base64Pem { content }
    }
}

impl From<&PrivateKeyChain> for Base64Pem {
    fn from(key: &PrivateKeyChain) -> Self {
        let pem = Pem::new("RSA PRIVATE KEY", key.key());
        let content = encode(&pem);
        Base64Pem { content }
    }
}

use std::fmt::Display;

use ::pem::{encode, Pem};
use color_eyre::Result;
use p12_keystore::{Certificate, PrivateKeyChain};
use pem::parse_many;
use tracing::warn;
