use super::file_priority_state::FilePriorityState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down |";

pub struct FilePriorityComponent {
    pub state: FilePriorityState,
}

impl FilePriorityComponent {
    pub fn init() -> Self {
        FilePriorityComponent {
            state: FilePriorityState::init(),
        }
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
