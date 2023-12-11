use crate::application::{ app::App, app_mode::{ AppMode, DirPriorityForm } };

use super::dir_priority_form_state::DirPriorityFormState;

const HELP: &'static str = "";

pub struct DirPriorityFormComponent {
    pub state: DirPriorityFormState,
}

impl DirPriorityFormComponent {
    pub fn init() -> Self {
        DirPriorityFormComponent {
            state: DirPriorityFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: DirPriorityForm) {
        app.change_mode(AppMode::DirPriority, AppMode::DirPriorityForm(prev_mode));
    }

    pub fn next(app: &mut App, next: DirPriorityForm, prev_mode: DirPriorityForm) {
        app.change_mode(
            AppMode::DirPriorityForm(next),
            AppMode::DirPriorityForm(prev_mode)
        );
    }

    pub fn get_help_text(&self) -> &'static str {
        HELP
    }
}
