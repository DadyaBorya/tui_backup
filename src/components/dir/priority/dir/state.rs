use tui::widgets::ListState;

use crate::{models::entry_dir_priority::EntryDirPriority, utils::list_utils};

#[derive(Default)]
pub struct DirPriorityState {
    pub list_state: ListState,
    pub rules: Vec<EntryDirPriority>,
    pub is_edit: bool,
}

impl DirPriorityState {
    pub fn init() -> Self {
        DirPriorityState::default()
    }

    pub fn init_index_table(&mut self) {
        let len = self.rules.len();
        list_utils::init_index_table(&mut self.list_state, len);
    }

    pub fn rows(&self) -> Vec<String> {
        self.rules
            .iter()
            .map(|rule| rule.row())
            .collect()
    }
}
