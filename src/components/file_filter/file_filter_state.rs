use tui::widgets::ListState;

use crate::{ models::entry_file_filter::EntryFileFilter, utils::list_utils };

#[derive(Default)]
pub struct FileFilterState {
    pub list_state: ListState,
    pub rules: Vec<EntryFileFilter>,
    pub is_edit: bool,
}

impl FileFilterState {
    pub fn init() -> Self {
        FileFilterState::default()
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
