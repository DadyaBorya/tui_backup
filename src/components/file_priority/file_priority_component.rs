use crate::application::{app::App, app_mode::AppMode};

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

    pub fn exit(app: &mut App) {
        let file_priority = &mut app.components.file_priority;
        file_priority.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::FilePriority);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
