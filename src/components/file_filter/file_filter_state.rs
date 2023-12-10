use tui::widgets::ListState;

use crate::models::entry_file_filter::EntryFileFilter;

#[derive(Default)]
pub struct FileFilterState {
    pub list_state: ListState,
    pub rules: Vec<EntryFileFilter>,
}

impl FileFilterState {
    pub fn init() -> Self {
        FileFilterState::default()
    }

    pub fn rows(&self) -> Vec<String> {
        self.rules.iter().map(|rule| rule.row()).collect()
    }
}
