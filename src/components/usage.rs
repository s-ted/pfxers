#[set]
pub(crate) fn WUsage(app: App) -> App {
    app.widgets(widget)
}

fn widget(mut frame: ResMut<WidgetFrame>, chunks: Res<Chunks>) -> WidgetResult {
    let area = chunks.get_chunk::<WUsage>()?;

    frame.render_widget(
        Paragraph::new(
            "Esc to quit, C to copy current cert, I to copy all certs, Up/Down to navigate \
             certificate chain",
        )
        .style(Style::default().bg(Color::White).fg(Color::Black)),
        area,
    );

    Ok(())
}

use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use widgetui::*;
