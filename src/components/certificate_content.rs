#[set]
pub(crate) fn WCertificateContent(app: App) -> App {
    app.widgets(widget)
}

fn widget(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<AppState>,
) -> WidgetResult {
    let area = chunks.get_chunk::<WCertificateContent>()?;

    if let Some(cert) = state.certs.get(state.current_cert_index) {
        let content = Base64Pem::from(cert).to_string();
        frame.render_widget(Paragraph::new(content), area);
    }

    Ok(())
}

use ratatui::widgets::Paragraph;
use widgetui::*;

use crate::app::AppState;
use crate::pem::Base64Pem;
