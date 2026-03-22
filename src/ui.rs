use crate::app::{AppState, PaneFocus};
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, List, ListItem, Paragraph},
};

pub fn view(frame: &mut Frame, state: &mut AppState) {
    let area = frame.area();
    let active_style = Style::default().fg(Color::Yellow);
    let inactive_style = Style::default().fg(Color::Gray);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[0]);

    let left_border = if matches!(state.focus, PaneFocus::GameList) {
        active_style
    } else {
        inactive_style
    };

    let right_border = if matches!(state.focus, PaneFocus::ModList) {
        active_style
    } else {
        inactive_style
    };

    let game_list_items: Vec<ListItem> = state
        .get_games()
        .iter()
        .map(|game| ListItem::new(game.name.clone()))
        .collect();

    let game_list = List::new(game_list_items)
        .block(Block::bordered().title("Games").border_style(left_border))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("> ");

    frame.render_stateful_widget(game_list, body_chunks[0], state.get_game_list_state());

    let status_text = state.status_message.as_deref().unwrap_or("Ready");
    let status_bar = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Left);

    frame.render_widget(status_bar, main_layout[1]);

    match state.active_game_index {
        Some(index) => {
            if let Some(game) = state.games.get(index) {
                match &game.mods_path {
                    Some(_path) => {
                        let mod_items: Vec<ListItem> = game
                            .mods
                            .iter()
                            .map(|m| {
                                let color = if m.enabled {
                                    Color::Green
                                } else {
                                    Color::White
                                };
                                ListItem::new(m.name.clone()).style(Style::default().fg(color))
                            })
                            .collect();

                        let mod_list = List::new(mod_items)
                            .block(
                                Block::bordered()
                                    .title(format!(" Managing Mods: {} ", game.name))
                                    .border_style(right_border),
                            )
                            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                            .highlight_symbol("> ");
                        frame.render_stateful_widget(
                            mod_list,
                            body_chunks[1],
                            &mut state.mod_list_state,
                        );
                    }
                    None => {
                        let prompt = Paragraph::new("\nNo mod folder is linked for this game.\n\nPress <m> to link a directory")
                            .alignment(Alignment::Center)
                            .block(Block::bordered().title(format!(" Managing Mods: {} ", game.name)).border_style(right_border));
                        frame.render_widget(prompt, body_chunks[1])
                    }
                }
            }
        }
        None => {
            let help_text = Paragraph::new("Welcome to Termite!\n\nSelect a game on the left and press <Enter> to manage mods.")
                .block(Block::bordered().title(" Info "));
            frame.render_widget(help_text, body_chunks[1]);
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
