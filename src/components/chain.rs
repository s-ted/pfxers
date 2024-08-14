#[set]
pub(crate) fn WChain(app: App) -> App {
    app.widgets(widget)
}

fn widget(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<AppState>,
) -> WidgetResult {
    let area = chunks.get_chunk::<WChain>()?;

    let items: List = state
        .certs
        .iter()
        .enumerate()
        .map(|(index, cert)| {
            if index == state.current_cert_index {
                Line::default().spans([">> ".to_string().red(), cert.subject().bold()])
            } else {
                Line::default().spans(["   ".to_string().red(), cert.subject().into()])
            }
        })
        .collect();

    let list = items.block(Block::bordered().title("Chain"));
    frame.render_widget(list, area);

    Ok(())
}

use ratatui::style::Stylize as _;
use ratatui::text::Line;
use ratatui::widgets::{Block, List};
use widgetui::*;

use crate::app::AppState;
