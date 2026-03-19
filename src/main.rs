use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::error::Error;
use std::io;

mod app;
mod event;
mod ui;

use crate::app::{AppState, Message};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    //Setup panic hook to clean up terminal if the app crashes
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        panic_hook(panic_info);
    }));

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
        "Item 4".to_string(),
        "Item 5".to_string(),
    ]);

    // For listing out steam games, use:
    //ls -1 /home/mentat/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/ | grep -v -E "(Proton|Steam)"
    //set initial state to item 1

    loop {
        terminal.draw(|frame| ui::view(frame, &mut state))?;

        if let Some(msg) = event::handle_event()? {
            if matches!(msg, Message::Quit) {
                break;
            }
            state.update(msg);
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
