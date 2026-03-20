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
        .get_games()
        .iter()
        .map(|game| ListItem::new(game.name.clone()))
        .collect();

    let game_list = List::new(game_list_items)
        .block(Block::bordered().title("Games"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("> ");

    frame.render_stateful_widget(game_list, chunks[0], state.get_game_list_state());

    match state.active_game_index {
        Some(index) => {
            if let Some(game) = state.games.get(index) {
                let mod_info = match &game.mods_path {
                    Some(path) => {
                        format!("Linked Mod Folder: {:?}\n\n[Mods List Placeholder]", path)
                    }
                    None => {
                        "No mods folder linked.\n\nPress 'm' to link a mod folder for this game."
                            .to_string()
                    }
                };
                let mod_view = Paragraph::new(mod_info)
                    .block(Block::bordered().title(format!(" Managing Mods: {} ", game.name)));
                frame.render_widget(mod_view, chunks[1]);
            }
        }
        None => {
            let help_text = Paragraph::new("Welcome to Termite!\n\nSelect a game on the left and press <Enter> to manage mods.")
                .block(Block::bordered().title(" Info "));
            frame.render_widget(help_text, chunks[1]);
        }
    }

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
