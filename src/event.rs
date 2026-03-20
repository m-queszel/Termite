use crossterm::event::{self, Event, KeyCode};

use crate::app::{AppState, Message};

pub fn handle_event(state: &mut AppState) -> Result<Option<Message>, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(16))?
        && let Event::Key(key) = event::read()?
    {
        if state.explorer.is_some() {
            return Ok(Some(match key.code {
                KeyCode::Esc => Message::CloseDialog,
                KeyCode::Enter => {
                    if let Some(explorer) = &state.explorer {
                        let i = explorer.list_state.selected().unwrap_or(0);
                        let selected_name = &explorer.items[i];
                        let full_path = explorer.path.join(selected_name);
                        Message::SelectPath(full_path)
                    } else {
                        Message::CloseDialog
                    }
                }
                KeyCode::Char('j') => Message::MoveDown,
                KeyCode::Char('k') => Message::MoveUp,
                KeyCode::Char('h') => Message::GoBackFromDirectory,
                KeyCode::Char('l') => Message::EnterDirectory,
                _ => return Ok(None),
            }));
        }

        if state.active_game_index.is_some() {
            match key.code {
                KeyCode::Char('m') => return Ok(Some(Message::OpenModDialog)),
                _ => {}
            }
        }
        return Ok(Some(match key.code {
            KeyCode::Char('q') => Message::Quit,
            KeyCode::Char('j') => Message::MoveDown,
            KeyCode::Char('k') => Message::MoveUp,
            KeyCode::Char('d') => Message::OpenDialog,
            KeyCode::Enter => Message::ModGame,
            _ => return Ok(None),
        }));
    }
    Ok(None)
}
