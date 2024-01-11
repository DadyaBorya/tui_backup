use tui::widgets::ListState;

use crate::utils::list_utils;

const SETTINGS: [&str; 4] = ["File Filter", "Dir File Priority", "Dir Priority", "File Priority"];

#[derive(Default)]
pub struct FileListSettingState {
    pub list_state: ListState,
    pub seleted_items: Vec<usize>,
}

impl FileListSettingState {
    pub fn init() -> Self {
        let mut state = FileListSettingState::default();
        state.seleted_items.extend_from_slice(&[0, 1, 2, 3]);
        state
    }

    pub fn init_index_table(&mut self) {
        let len = self.rows().len();
        list_utils::init_index_table(&mut self.list_state, len);
    }

    pub fn rows(&self) -> Vec<String> {
        let mut rows: Vec<String> = vec![];
        for (index, str) in SETTINGS.iter().enumerate() {
            match self.seleted_items.contains(&index) {
                true => rows.push(format!("[x] {}", str)),
                false => rows.push(format!("[ ] {}", str)),
            }
        }

        rows
    }
}
