#[set]
pub(crate) fn WCertificateDetails(app: App) -> App {
    app.widgets(widget)
}

fn widget(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<AppState>,
) -> WidgetResult {
    let area = chunks.get_chunk::<WCertificateDetails>()?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unable to get current time")
        .as_secs() as i64;

    let layout = layout![
        area,
        (#1),
        (#1),
        (#1),
        (%100)
    ];

    if let Some(cert) = state.certs.get(state.current_cert_index) {
        let subject = cert.subject().bold();

        let mut validity_line = vec![];
        let mut sans_list = vec![];

        {
            use x509_parser::prelude::*;
            if let Ok((_rem, cert)) = X509Certificate::from_der(cert.as_der()) {
                let not_before = cert.validity().not_before.to_string().green();
                let not_after = cert.validity.not_after.to_string().red();
                let validity = if cert.validity.not_after.timestamp() < now {
                    "EXPIRED".red().bold()
                } else {
                    "✅".green()
                };
                validity_line = vec![validity, " ".into(), not_before, " -> ".into(), not_after];

                if let Ok(sans) = cert.subject_alternative_name() {
                    if let Some(sans) = sans.map(|x| x.value.general_names.clone()) {
                        sans_list = sans;
                    }
                };
            }
        }

        frame.render_widget(Line::default().spans(["Subject: ".into(), subject]), layout[0][0]);
        frame.render_widget(
            Line::default().spans(["Validity: ".into()].into_iter().chain(validity_line)),
            layout[1][0],
        );

        let issuer = if cert.issuer() == cert.subject() {
            "self-signed".red().bold()
        } else {
            cert.issuer().blue()
        };
        frame.render_widget(Line::default().spans(["Issuer: ".into(), issuer]), layout[2][0]);

        let sans: Vec<_> = sans_list.iter().map(|x| Line::from(x.to_string())).collect();
        frame.render_widget(Paragraph::new(sans), layout[3][0]);
    }

    Ok(())
}

use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use widgetui::*;

use crate::app::AppState;
