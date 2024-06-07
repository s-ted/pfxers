pub(crate) fn parse_pem<'a>(input: &str) -> Result<(Option<PrivateKeyChain>, Vec<Certificate>)> {
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

pub(crate) fn show_info(key: Option<&PrivateKeyChain>, certs: &[Certificate]) {
    if let Some(x) = key {
        println!(
            "[{kind}] for {subject}",
            kind = "KEY".red().bold(),
            subject = x.chain().first().map(|cert| cert.subject()).unwrap_or("???").bold()
        )
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unable to get current time")
        .as_secs() as i64;

    certs.iter().for_each(|cert| {
        println!(
            "[{kind}] for {subject} ({issuer})",
            kind = "CERT".green().bold(),
            subject = cert.subject().to_string().bold(),
            issuer = if cert.issuer() == cert.subject() {
                "self-signed".red().bold()
            } else {
                format!("issued by {}", cert.issuer()).into()
            }
        );

        {
            use x509_parser::prelude::*;
            if let Ok((_rem, cert)) = X509Certificate::from_der(cert.as_der()) {
                let not_before = cert.validity().not_before;
                let not_after = cert.validity.not_after;
                let validity = if cert.validity.not_after.timestamp() < now {
                    "EXPIRED".red().bold()
                } else {
                    "✅".green()
                };
                println!("  {not_before:} -> {not_after:} {validity:}",);

                if let Ok(sans) = cert.subject_alternative_name() {
                    if let Some(sans) = sans.map(|x| x.value.general_names.clone()) {
                        if !sans.is_empty() {
                            let sans = sans
                                .iter()
                                .map(|x| format!("{}", x.to_string()))
                                .collect::<Vec<_>>()
                                .join(", ");

                            println!("  [{kind}] {sans}", kind = "SANS".blue(),);
                        }
                    }
                }
            }
        }
    });
}

use anyhow::Result;
use colored::Colorize as _;
use log::warn;
use p12_keystore::{Certificate, PrivateKeyChain};
use pem::parse_many;
