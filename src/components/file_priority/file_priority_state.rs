use tui::widgets::ListState;

use crate::{models::entry_file_priority::EntryFilePriority, utils::list_utils};

#[derive(Default)]
pub struct FilePriorityState {
    pub list_state: ListState,
    pub rules: Vec<EntryFilePriority>,
    pub is_edit: bool,
}

impl FilePriorityState {
    pub fn init() -> Self {
        FilePriorityState::default()
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
