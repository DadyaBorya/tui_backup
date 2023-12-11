use crate::application::{ app::App, app_mode::{ DirFilePriorityForm, AppMode } };

use super::dir_file_priority_form_state::DirFilePriorityFormState;

const HELP: &'static str = "";

pub struct DirFilePriorityFormComponent {
    pub state: DirFilePriorityFormState,
}

impl DirFilePriorityFormComponent {
    pub fn init() -> Self {
        DirFilePriorityFormComponent {
            state: DirFilePriorityFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: DirFilePriorityForm) {
        app.change_mode(AppMode::DirFilePriority, AppMode::DirFilePriorityForm(prev_mode));
    }

    pub fn next(app: &mut App, next: DirFilePriorityForm, prev_mode: DirFilePriorityForm) {
        app.change_mode(
            AppMode::DirFilePriorityForm(next),
            AppMode::DirFilePriorityForm(prev_mode)
        );
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
