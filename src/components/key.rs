#[set]
pub(crate) fn WKey(app: App) -> App {
    app.widgets(widget)
}

fn widget(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<AppState>,
) -> WidgetResult {
    let area = chunks.get_chunk::<WKey>()?;

    if state.key.is_some() {
        frame.render_widget(
            Paragraph::new("This file contains a key: press K to copy it to keyboard")
                .style(Style::default().bg(Color::White).fg(Color::Black)),
            area,
        );
    }

    Ok(())
}

use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use widgetui::*;

use crate::app::AppState;
