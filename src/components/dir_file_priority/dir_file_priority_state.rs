use tui::widgets::ListState;

use crate::{models::entry_dir_file_priority::EntryDirFilePriority, utils::list_utils};

#[derive(Default)]
pub struct DirFilePriorityState {
    pub list_state: ListState,
    pub rules: Vec<EntryDirFilePriority>,
    pub is_edit: bool,
}

impl DirFilePriorityState {
    pub fn init() -> Self {
        DirFilePriorityState::default()
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
