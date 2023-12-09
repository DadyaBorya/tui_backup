use super::file_filter_state::FileFilterState;

const HELP: &'static str = "| ↑ Up | ↓ Down | TAB~Next |";

pub struct FileFilterComponent {
    pub state: FileFilterState,
}

impl FileFilterComponent {
    pub fn init() -> Self {
        FileFilterComponent {
            state: FileFilterState::init(),
        }
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
