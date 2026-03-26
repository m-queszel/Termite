use std::path::PathBuf;

use ratatui::widgets::ListState;

use crate::config;
use crate::directory_manager;
use crate::models::game::InjectionStrategy;
use crate::models::game::{Game, Mod};
use crate::symlink_manager;

pub enum Message {
    ToggleFocus,
    ToggleMod,
    MoveUp,
    MoveDown,
    Quit,
    OpenDialog,
    OpenModDialog,
    CloseDialog,
    SelectPath(PathBuf),
    EnterDirectory,
    GoBackFromDirectory,
    ModGame,
    OpenInjectionDialog,
    ConfirmPermission,
    CancelPermission,
}

pub enum ExplorerPurpose {
    SelectGameLibrary,
    SelectModFolder,
    SelectInjectionPath,
}

pub enum PaneFocus {
    GameList,
    ModList,
}

pub struct ExplorerState {
    pub path: PathBuf,
    pub items: Vec<String>,
    pub list_state: ListState,
    pub history: Vec<usize>,
    pub purpose: ExplorerPurpose,
}

pub struct AppState {
    pub games: Vec<Game>,
    pub game_list_state: ListState,
    pub mod_list_state: ListState,
    pub explorer: Option<ExplorerState>,
    pub active_game_index: Option<usize>,
    pub status_message: Option<String>,
    pub focus: PaneFocus,
    pub pending_permission_path: Option<PathBuf>,
    pub show_permission_prompt: bool,
}

impl AppState {
    pub fn new(games: Vec<Game>) -> Self {
        let mut game_list_state = ListState::default();
        game_list_state.select(Some(0));

        AppState {
            game_list_state,
            mod_list_state: ListState::default(),
            games,
            explorer: None,
            active_game_index: None,
            focus: PaneFocus::GameList,
            status_message: Some("Welcome to Termite!".to_string()),
            pending_permission_path: None,
            show_permission_prompt: false,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::OpenInjectionDialog => {
                self.explorer = self.create_explorer(ExplorerPurpose::SelectInjectionPath)
            }
            Message::OpenDialog => {
                self.explorer = self.create_explorer(ExplorerPurpose::SelectGameLibrary)
            }
            Message::OpenModDialog => {
                self.explorer = self.create_explorer(ExplorerPurpose::SelectModFolder)
            }
            Message::EnterDirectory => {
                if let Some(explorer) = &mut self.explorer {
                    let index = explorer.list_state.selected().unwrap_or(0);
                    let directory_name = &explorer.items[index];
                    let new_path = explorer.path.join(directory_name);
                    if new_path.is_dir() {
                        explorer.history.push(index);
                        explorer.path = new_path;
                        explorer.items = directory_manager::list_directory_contents(&explorer.path);
                        explorer.list_state.select(Some(0));
                    }
                }
            }
            Message::ConfirmPermission => {
                if let Some(path) = self.pending_permission_path.take() {
                    match directory_manager::grant_flatpak_permission(&path) {
                        Ok(_) => {
                            self.status_message = Some("Flatpak permission granted!".to_string())
                        }
                        Err(e) => self.status_message = Some(format!("Permission error: {}", e)),
                    }
                }
                self.show_permission_prompt = false;
            }
            Message::CancelPermission => {
                self.pending_permission_path = None;
                self.show_permission_prompt = false;
                self.status_message = Some("Permission denied. Mods may not work.".to_string());
            }
            Message::GoBackFromDirectory => {
                if let Some(explorer) = &mut self.explorer
                    && let Some(parent_path) =
                        explorer.path.parent().map(|parent| parent.to_path_buf())
                {
                    explorer.path = parent_path;
                    explorer.items = directory_manager::list_directory_contents(&explorer.path);
                    explorer.list_state.select(explorer.history.pop());
                }
            }
            Message::ModGame => {
                self.active_game_index = self.game_list_state.selected();
                if self.active_game_index.is_some() {
                    self.focus = PaneFocus::ModList;
                    self.mod_list_state.select(Some(0));
                }
            }
            Message::CloseDialog => self.explorer = None,
            Message::SelectPath(ref path) => {
                if let Some(explorer) = &self.explorer {
                    match explorer.purpose {
                        ExplorerPurpose::SelectInjectionPath => {
                            if let Some(index) = self.active_game_index
                                && let Some(game) = self.games.get_mut(index)
                            {
                                game.mods_path = Some(path.clone());
                                self.status_message = Some(format!(
                                    "Mods folder set to: {:?}",
                                    path.file_name().unwrap_or_default()
                                ));
                            }
                            self.explorer = None;
                        }
                        ExplorerPurpose::SelectGameLibrary => {
                            let folder_names = directory_manager::list_directory_contents(path);
                            self.games = folder_names
                                .into_iter()
                                .map(|name| {
                                    let full_path = path.join(&name);

                                    // Auto-discovery of mods folder
                                    let mut mods_path = None;
                                    let potential_mods = full_path.join("Mods");
                                    if potential_mods.exists() && potential_mods.is_dir() {
                                        mods_path = Some(potential_mods);
                                    } else {
                                        // Case-insensitive check for linux
                                        let potential_mods_lower = full_path.join("mods");
                                        if potential_mods_lower.exists()
                                            && potential_mods_lower.is_dir()
                                        {
                                            mods_path = Some(potential_mods_lower);
                                        }
                                    }

                                    Game {
                                        name,
                                        path: full_path,
                                        mods_path,
                                        mods: Vec::new(),
                                    }
                                })
                                .collect();
                            self.explorer = None;
                        }
                        ExplorerPurpose::SelectModFolder => {
                            if let Some(index) = self.active_game_index
                                && let Some(game) = self.games.get_mut(index)
                            {
                                let mod_names = directory_manager::list_directory_contents(path);
                                game.mods = mod_names
                                    .into_iter()
                                    .filter(|name| path.join(name).is_dir())
                                    .map(|name| {
                                        let full_path = path.join(&name);
                                        Mod {
                                            name: name.clone(),
                                            path: full_path,
                                            enabled: false,
                                            injection_method: InjectionStrategy::AddAsFolder(
                                                name.into(),
                                            ),
                                        }
                                    })
                                    .collect();

                                if directory_manager::is_flatpak_game(&game.path) {
                                    self.pending_permission_path = Some(path.clone());
                                    self.show_permission_prompt = true;
                                }
                            }
                            self.explorer = None;
                        }
                    }
                }
            }
            Message::ToggleFocus => {
                self.focus = match self.focus {
                    PaneFocus::GameList => PaneFocus::ModList,
                    PaneFocus::ModList => PaneFocus::GameList,
                };
                if matches!(self.focus, PaneFocus::ModList)
                    && self.mod_list_state.selected().is_none()
                {
                    self.mod_list_state.select(Some(0));
                }
            }
            Message::ToggleMod => {
                if let Some(game_index) = self.active_game_index
                    && let Some(mod_index) = self.mod_list_state.selected()
                {
                    let (is_enabled, mod_name) = {
                        let game = &mut self.games[game_index];
                        let m = &mut game.mods[mod_index];
                        m.enabled = !m.enabled;
                        (m.enabled, m.name.clone())
                    };

                    let game = &self.games[game_index];
                    let result = if is_enabled {
                        symlink_manager::apply_mod(game, mod_index)
                    } else {
                        symlink_manager::remove_mod(game, mod_index)
                    };

                    match result {
                        Ok(_) => {
                            self.status_message = Some(format!(
                                "Successfully {} mod: {}",
                                if is_enabled { "applied" } else { "removed" },
                                mod_name
                            ))
                        }
                        Err(e) => {
                            self.status_message = Some(format!(
                                "Error {} mod: {}",
                                if is_enabled { "applying" } else { "removing" },
                                e
                            ))
                        }
                    }
                }
            }
            Message::MoveUp => self.move_up(),
            Message::MoveDown => self.move_down(),
            Message::Quit => {}
        }
        if self.should_save(&msg) {
            let _ = config::save(&self.games);
        }
    }

    pub fn move_down(&mut self) {
        if let Some(explorer) = &mut self.explorer {
            let i = explorer.list_state.selected().unwrap_or(0);
            if i < explorer.items.len().saturating_sub(1) {
                explorer.list_state.select(Some(i + 1));
            }
        } else {
            match self.focus {
                PaneFocus::GameList => {
                    let i = self.game_list_state.selected().unwrap_or(0);
                    if i < self.games.len().saturating_sub(1) {
                        self.game_list_state.select(Some(i + 1));
                    }
                }
                PaneFocus::ModList => {
                    if let Some(game) = self
                        .active_game_index
                        .and_then(|index| self.games.get(index))
                    {
                        let i = self.mod_list_state.selected().unwrap_or(0);
                        if i < game.mods.len().saturating_sub(1) {
                            self.mod_list_state.select(Some(i + 1));
                        }
                    }
                }
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
            match self.focus {
                PaneFocus::GameList => {
                    let i = self.game_list_state.selected().unwrap_or(0);
                    if i > 0 {
                        self.game_list_state.select(Some(i - 1));
                    }
                }
                PaneFocus::ModList => {
                    let i = self.mod_list_state.selected().unwrap_or(0);
                    if i > 0 {
                        self.mod_list_state.select(Some(i - 1));
                    }
                }
            }
        }
    }

    fn create_explorer(&self, purpose: ExplorerPurpose) -> Option<ExplorerState> {
        let path = home::home_dir()?;
        let items = directory_manager::list_directory_contents(&path);
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Some(ExplorerState {
            path,
            items,
            list_state,
            history: Vec::new(),
            purpose,
        })
    }

    fn should_save(&self, msg: &Message) -> bool {
        match msg {
            Message::ToggleMod | Message::SelectPath(_) => true,
            _ => false,
        }
    }

    pub fn get_games(&self) -> &Vec<Game> {
        &self.games
    }

    pub fn get_game_list_state(&mut self) -> &mut ListState {
        &mut self.game_list_state
    }
}
