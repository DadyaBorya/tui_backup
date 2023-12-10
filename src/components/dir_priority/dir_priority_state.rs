use tui::widgets::ListState;

use crate::models::entry_dir_priority::EntryDirPriority;

#[derive(Default)]
pub struct DirPriorityState {
    pub list_state: ListState,
    pub rules: Vec<EntryDirPriority>,
}

impl DirPriorityState {
    pub fn init() -> Self {
        DirPriorityState::default()
    }

    pub fn rows(&self) -> Vec<String> {
        self.rules
            .iter()
            .map(|rule| rule.row())
            .collect()
    }
}
