use crate::application::{ app::App, app_mode::AppMode };

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

    pub fn exit(app: &mut App) {
        let dir_priority = &mut app.components.dir_priority;
        dir_priority.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::DirFilePriority);
    }

    pub fn prev_component(app: &mut App) {
        let dir_priority = &mut app.components.dir_priority;
        dir_priority.state.list_state.select(None);
        app.change_mode(AppMode::DirFilePriority, AppMode::DirFilePriority);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
