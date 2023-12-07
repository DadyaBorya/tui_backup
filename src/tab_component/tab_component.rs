use crate::utils::tab_movement;

use super::tab_state::TabState;

const HELP: &'static str = "| <- Prev | -> Next | | ENTER~Select | q~Exit |";

pub struct TabComponent {
    pub state: TabState,
}

impl TabComponent {
    pub fn init() -> Self {
        TabComponent { state: TabState::init() }
    }

    pub fn next(&mut self) {
        let index = tab_movement::next(self.state.index, self.state.headers.len());
        self.state.index = index;
    }

    pub fn previous(&mut self) {
        let index = tab_movement::previous(self.state.index, self.state.headers.len());
        self.state.index = index;
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
