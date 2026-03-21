use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::error::Error;
use std::io;

mod app;
mod directory_manager;
mod event;
mod ui;
mod models {
    pub mod game;
}

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

    let steam_root = directory_manager::find_steam_root();
    let steam_games = match steam_root {
        Some(path) => directory_manager::list_steam_games(path),
        None => vec![],
    };

    let mut state = AppState::new(steam_games);

    loop {
        terminal.draw(|frame| ui::view(frame, &mut state))?;

        if let Some(msg) = event::handle_event(&mut state)? {
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
