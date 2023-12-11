use crate::application::{ app::App, app_mode::{ AppMode, DirPriorityForm } };

use super::dir_priority_form_state::DirPriorityFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_DEEP: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_PRIORITY: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

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

    pub fn get_help_text(&self, mode: &DirPriorityForm) -> &'static str {
        match mode {
            DirPriorityForm::Regex => HELP_REGEX,
            DirPriorityForm::Deep => HELP_DEEP,
            DirPriorityForm::Priority => HELP_PRIORITY,
            DirPriorityForm::Submit => HELP_SUBMIT
        }
    }
}
