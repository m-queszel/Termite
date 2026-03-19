use crossterm::event::{self, Event, KeyCode};

use crate::app::Message;

pub fn handle_event() -> Result<Option<Message>, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(16))?
        && let Event::Key(key) = event::read()?
    {
        return Ok(Some(match key.code {
            KeyCode::Char('q') => Message::Quit,
            KeyCode::Char('j') => Message::MoveDown,
            KeyCode::Char('k') => Message::MoveUp,
            _ => return Ok(None),
        }));
    }
    Ok(None)
}
