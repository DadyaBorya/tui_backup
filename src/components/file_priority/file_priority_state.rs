use tui::widgets::ListState;

use crate::models::entry_file_priority::EntryFilePriority;

#[derive(Default)]
pub struct FilePriorityState {
    pub list_state: ListState,
    pub rules: Vec<EntryFilePriority>,
}

impl FilePriorityState {
    pub fn init() -> Self {
        FilePriorityState::default()
    }

    pub fn rows(&self) -> Vec<String> {
        self.rules
            .iter()
            .map(|rule| rule.row())
            .collect()
    }
}
