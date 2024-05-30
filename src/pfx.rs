pub(crate) fn parse_fpx(
    pfx_file_path: &str,
    password_file_path: Option<&str>,
) -> Result<(Option<PrivateKeyChain>, Vec<Certificate>)> {
    let pfx_content = std::fs::read(pfx_file_path)?;
    let password = password_file_path
        .map(std::fs::read_to_string)
        .unwrap_or_else(|| rpassword::prompt_password("PFX file password: "))
        .unwrap_or_default();

    let pfx = KeyStore::from_pkcs12(&pfx_content, password.as_str())?;

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

pub(crate) fn show_info(key: Option<&PrivateKeyChain>, certs: &[Certificate]) {
    if let Some(x) = key {
        println!(
            "[{kind}] for {subject}",
            kind = "KEY".red().bold(),
            subject = x.chain().first().map(|cert| cert.subject()).unwrap_or("???").bold()
        )
    }

    certs.iter().for_each(|cert| {
        println!(
            "[{kind}] for {subject} ({issuer})",
            kind = "CERT".green().bold(),
            subject = cert.subject().bold(),
            issuer = if cert.issuer() == cert.subject() {
                "self-signed".red().bold()
            } else {
                format!("issued by {}", cert.issuer()).into()
            }
        );

        {
            use x509_parser::prelude::*;
            if let Ok((_rem, cert)) = X509Certificate::from_der(cert.as_der()) {
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

pub(crate) fn show_pem(key: Option<&PrivateKeyChain>, certs: &[Certificate]) {
    if let Some(key) = key {
        println!(
            "[{kind}] for {subject}",
            kind = "KEY".red().bold(),
            subject = key.chain().first().map(|cert| cert.subject()).unwrap_or("???").bold()
        );
        let pem = Pem::new("PRIVATE KEY", key.key());
        println!("{}", encode(&pem));
    }

    certs.iter().for_each(|cert| {
        println!(
            "[{kind}] for {subject} ({issuer})",
            kind = "CERT".green().bold(),
            subject = cert.subject().bold(),
            issuer = if cert.issuer() == cert.subject() {
                "self-signed".red().bold()
            } else {
                format!("issued by {}", cert.issuer()).into()
            }
        );
        let pem = Pem::new("CERTIFICATE", cert.as_der());
        println!("{}", encode(&pem));
    });
}

use anyhow::Result;
use colored::Colorize as _;
use p12_keystore::{Certificate, KeyStore, KeyStoreEntry, PrivateKeyChain};
use pem::{encode, Pem};
