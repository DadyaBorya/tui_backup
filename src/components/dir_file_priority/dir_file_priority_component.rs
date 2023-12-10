use crate::application::{app::App, app_mode::AppMode};

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

    pub fn exit(app: &mut App) {
        let dir_file_priority = &mut app.components.dir_file_priority;
        dir_file_priority.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::DirFilePriority);
    }

    pub fn next_component(app: &mut App) {
        let dir_file_priority = &mut app.components.dir_file_priority;
        dir_file_priority.state.list_state.select(None);
        app.change_mode(AppMode::DirPriority, AppMode::DirFilePriority);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
