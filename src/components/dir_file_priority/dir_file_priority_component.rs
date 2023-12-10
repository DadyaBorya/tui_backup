use super::dir_file_priority_state::DirFilePriorityState;

const HELP: &'static str = "| ESC~Back \
| ↑ Up | ↓ Down | ]~Next |";

pub struct DirFilePriorityComponent {
    pub state: DirFilePriorityState,
}

impl DirFilePriorityComponent {
    pub fn init() -> Self {
        DirFilePriorityComponent {
            state: DirFilePriorityState::init(),
        }
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
