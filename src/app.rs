use std::path::PathBuf;

use ratatui::widgets::ListState;

use crate::directory_manager;

pub enum Message {
    MoveUp,
    MoveDown,
    Quit,
    OpenDialog,
    CloseDialog,
    SelectPath(PathBuf),
}

pub struct ExplorerState {
    pub path: PathBuf,
    pub items: Vec<String>,
    pub list_state: ListState,
}

pub struct AppState {
    pub game_items: Vec<String>,
    pub game_list_state: ListState,
    pub explorer: Option<ExplorerState>,
}

impl AppState {
    pub fn new(items: Vec<String>) -> Self {
        let mut game_list_state = ListState::default();
        game_list_state.select(Some(0));

        AppState {
            game_list_state,
            game_items: items,
            explorer: None,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::OpenDialog => {
                if let Some(path) = home::home_dir() {
                    let items = directory_manager::list_directory_contents(path.clone());
                    let mut list_state = ListState::default();
                    list_state.select(Some(0));
                    self.explorer = Some(ExplorerState {
                        path,
                        items,
                        list_state,
                    });
                }
            }
            Message::CloseDialog => self.explorer = None,
            Message::SelectPath(path) => {
                self.game_items = directory_manager::list_directory_contents(path);
                self.explorer = None;
            }
            Message::MoveUp => self.move_up(),
            Message::MoveDown => self.move_down(),
            Message::Quit => {}
        }
    }

    pub fn move_down(&mut self) {
        if let Some(explorer) = &mut self.explorer {
            let i = explorer.list_state.selected().unwrap_or(0);
            if i < explorer.items.len() - 1 {
                explorer.list_state.select(Some(i + 1));
            }
        } else {
            let i = self.game_list_state.selected().unwrap_or(0);
            if i < self.game_items.len() - 1 {
                self.game_list_state.select(Some(i + 1));
            }
        }
    }

    pub fn move_up(&mut self) {
        if let Some(explorer) = &mut self.explorer {
            let i = explorer.list_state.selected().unwrap_or(0);
            if i > 0 {
                explorer.list_state.select(Some(i - 1));
            }
        } else {
            let i = self.game_list_state.selected().unwrap_or(0);
            if i > 0 {
                self.game_list_state.select(Some(i - 1));
            }
        }
    }

    pub fn get_game_items(&self) -> &Vec<String> {
        &self.game_items
    }

    pub fn get_game_list_state(&mut self) -> &mut ListState {
        &mut self.game_list_state
    }
}
