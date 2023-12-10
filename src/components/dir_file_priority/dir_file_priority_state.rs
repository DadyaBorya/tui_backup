use tui::widgets::ListState;

use crate::models::entry_dir_file_priority::EntryDirFilePriority;

#[derive(Default)]
pub struct DirFilePriorityState {
    pub list_state: ListState,
    pub rules: Vec<EntryDirFilePriority>,
}

impl DirFilePriorityState {
    pub fn init() -> Self {
        DirFilePriorityState::default()
    }

    pub fn rows(&self) -> Vec<String> {
        self.rules
            .iter()
            .map(|rule| rule.row())
            .collect()
    }
}
