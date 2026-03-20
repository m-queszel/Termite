use crate::app::AppState;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, List, ListItem, Paragraph},
};

pub fn view(frame: &mut Frame, state: &mut AppState) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let game_list_items: Vec<ListItem> = state
        .get_game_items()
        .iter()
        .map(|item| ListItem::new(item.clone()))
        .collect();

    let game_list = List::new(game_list_items)
        .block(Block::bordered().title("Games"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("> ");

    frame.render_stateful_widget(game_list, chunks[0], state.get_game_list_state());

    frame.render_widget(
        Paragraph::new("Right View Widget").block(Block::bordered()),
        chunks[1],
    );

    if let Some(explorer) = &mut state.explorer {
        let popup_area = area.inner(Margin {
            vertical: 5,
            horizontal: 5,
        });

        frame.render_widget(Clear, popup_area);

        let explorer_items: Vec<ListItem> = explorer
            .items
            .iter()
            .map(|item| ListItem::new(item.clone()))
            .collect();

        let explorer_list = List::new(explorer_items)
            .block(
                Block::bordered()
                    .title(format!("Select Steam Directory: {:?}", explorer.path))
                    .bg(Color::Black),
            )
            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
            .highlight_symbol(">> ");

        frame.render_stateful_widget(explorer_list, popup_area, &mut explorer.list_state);
    }
}
