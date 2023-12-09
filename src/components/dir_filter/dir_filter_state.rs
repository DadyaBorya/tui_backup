use tui::widgets::ListState;

use crate::models::entry_dir_filter::EntryDirFilter;

#[derive(Default)]
pub struct DirFilterState {
    pub list_state: ListState,
    pub rules: Vec<EntryDirFilter>,
}

impl DirFilterState {
    pub fn init() -> Self {
        DirFilterState::default()
    }

    pub fn rows(&self) -> Vec<String> {
        self.rules
            .iter()
            .map(|rule| rule.row())
            .collect()
    }
}
