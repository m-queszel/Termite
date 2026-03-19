use ratatui::widgets::ListState;

pub enum Message {
    MoveUp,
    MoveDown,
    Quit,
}

pub struct AppState {
    list_state: ListState,
    items: Vec<String>,
}

impl AppState {
    pub fn new(items: Vec<String>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        AppState { list_state, items }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::MoveUp => self.move_up(),
            Message::MoveDown => self.move_down(),
            Message::Quit => {}
        }
    }

    pub fn move_down(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        if i < self.items.len() - 1 {
            self.list_state.select(Some(i + 1));
        }
    }

    pub fn move_up(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        if i > 0 {
            self.list_state.select(Some(i - 1));
        }
    }

    pub fn get_items(&self) -> &Vec<String> {
        &self.items
    }

    pub fn get_list_state(&mut self) -> &mut ListState {
        &mut self.list_state
    }
}
