use crate::app::AppState;
use ratatui::{
    prelude::*,
    widgets::{Block, List, ListItem, Paragraph},
};

pub fn view(frame: &mut Frame, state: &mut AppState) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let list_items: Vec<ListItem> = state
        .get_items()
        .iter()
        .map(|item| ListItem::new(item.clone()))
        .collect();

    let list = List::new(list_items)
        .block(Block::bordered().title("Items"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, chunks[0], state.get_list_state());

    frame.render_widget(
        Paragraph::new("Right View Widget").block(Block::bordered()),
        chunks[1],
    );
}
