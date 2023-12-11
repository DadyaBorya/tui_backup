use crate::application::{ app::App, app_mode::{ FilePriorityForm, AppMode } };

use super::file_priority_form_state::FilePriorityFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_CONTENT: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_PRIORITY: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

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

    pub fn get_help_text(&self, mode: &FilePriorityForm) -> &'static str {
        match mode {
            FilePriorityForm::Regex => HELP_REGEX,
            FilePriorityForm::Priority => HELP_PRIORITY,
            FilePriorityForm::Content => HELP_CONTENT,
            FilePriorityForm::Submit => HELP_SUBMIT,
        }
    }
}
