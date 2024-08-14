#[derive(State)]
pub(crate) struct AppState {
    pub key: Option<PrivateKeyChain>,
    pub certs: Vec<Certificate>,
    pub current_cert_index: usize,
}

impl TryFrom<Args> for AppState {
    type Error = color_eyre::Report;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let (key, certs) = match new_mime_guess::from_path(&args.input)
            .first_or_octet_stream()
            .essence_str()
        {
            "application/x-pkcs12" => {
                crate::pfx::parse_pfx(&args.input, args.password_file.as_deref(), args.password)?
            }

            "application/x-x509-ca-cert" => crate::pem::parse_pem(&args.input)?,

            x => {
                error!("Unknown mime type: {x}");
                Err(InputFileParsing::UnknownMimeType(x.to_string()))?
            }
        };

        Ok(AppState { key, certs, current_cert_index: 0 })
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum InputFileParsing {
    #[error("Unknown mime type: {0}")]
    UnknownMimeType(String),
}

#[set]
pub(crate) fn TheApp(app: App) -> App {
    app.widgets((chunk_builder, widget)).sets((
        WUsage,
        WChain,
        WCertificateContent,
        WCertificateDetails,
        WKey,
    ))
}

fn widget(mut events: ResMut<Events>, mut state: ResMut<AppState>) -> WidgetResult {
    if let Some(Event::Key(key)) = events.event {
        match key.into() {
            key!(down) => {
                if state.current_cert_index < state.certs.len() - 1 {
                    state.current_cert_index += 1
                }
            }

            key!(up) => {
                if state.current_cert_index > 0 {
                    state.current_cert_index -= 1
                }
            }

            key!(c) => {
                if let Some(cert) = state.certs.get(state.current_cert_index) {
                    use cli_clipboard::{ClipboardContext, ClipboardProvider};

                    if let Ok(mut ctx) = ClipboardContext::new() {
                        let content = Base64Pem::from(cert).to_string();
                        if let Err(e) = ctx.set_contents(content) {
                            error!("Unable to set clipboard content: {e}");
                        }
                    }
                }
            }

            key!(i) => {
                use cli_clipboard::{ClipboardContext, ClipboardProvider};

                if let Ok(mut ctx) = ClipboardContext::new() {
                    let content =
                        state.certs.iter().map(|cert| Base64Pem::from(cert).to_string()).collect();

                    if let Err(e) = ctx.set_contents(content) {
                        error!("Unable to set clipboard content: {e}");
                    }
                }
            }

            key!(k) => {
                if let Some(ref key) = state.key {
                    use cli_clipboard::{ClipboardContext, ClipboardProvider};

                    if let Ok(mut ctx) = ClipboardContext::new() {
                        let content = Base64Pem::from(key).to_string();
                        if let Err(e) = ctx.set_contents(content) {
                            error!("Unable to set clipboard content: {e}");
                        }
                    }
                }
            }

            key!(q) | key!(esc) | key!(ctrl - c) => events.register_exit(),

            _ => {}
        }
    }

    Ok(())
}

fn chunk_builder(frame: Res<'_, WidgetFrame>, mut chunks: ResMut<'_, Chunks>) -> WidgetResult {
    let layout = layout![
        frame.size(),
        (%20),
        (%20),
        (%80),
        (#1),
        (#1)
    ];

    chunks.register_chunk::<WCertificateDetails>(layout[0][0]);
    chunks.register_chunk::<WChain>(layout[1][0]);
    chunks.register_chunk::<WCertificateContent>(layout[2][0]);
    chunks.register_chunk::<WKey>(layout[3][0]);
    chunks.register_chunk::<WUsage>(layout[4][0]);

    Ok(())
}

use crokey::key;
use crossterm::event::Event;
use p12_keystore::{Certificate, PrivateKeyChain};
use ratatui::prelude::*;
use tracing::error;
use widgetui::*;

use crate::args::Args;
use crate::components::{WCertificateContent, WCertificateDetails, WChain, WKey, WUsage};
use crate::pem::Base64Pem;
