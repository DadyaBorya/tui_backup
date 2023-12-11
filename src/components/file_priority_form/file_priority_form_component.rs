use crate::application::{app::App, app_mode::{FilePriorityForm, AppMode}};

use super::file_priority_form_state::FilePriorityFormState;

const HELP: &'static str = "";

pub struct FilePriorityFormComponent {
    pub state: FilePriorityFormState,
}

impl FilePriorityFormComponent {
    pub fn init() -> Self {
        FilePriorityFormComponent {
            state: FilePriorityFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: FilePriorityForm) {
        app.change_mode(AppMode::FilePriority, AppMode::FilePriorityForm(prev_mode));
    }

    pub fn next(app: &mut App, next: FilePriorityForm, prev_mode: FilePriorityForm) {
        app.change_mode(AppMode::FilePriorityForm(next), AppMode::FilePriorityForm(prev_mode));
    }

    pub fn get_help_text(&self) -> &'static str {
        HELP
    }
}
