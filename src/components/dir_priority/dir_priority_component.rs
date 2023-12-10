use super::dir_priority_state::DirPriorityState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | [~Prev |";

pub struct DirPriorityComponent {
    pub state: DirPriorityState,
}

impl DirPriorityComponent {
    pub fn init() -> Self {
        DirPriorityComponent {
            state: DirPriorityState::init(),
        }
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
