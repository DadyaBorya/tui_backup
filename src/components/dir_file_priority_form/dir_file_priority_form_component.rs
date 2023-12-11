use crate::application::{ app::App, app_mode::{ DirFilePriorityForm, AppMode } };

use super::dir_file_priority_form_state::DirFilePriorityFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_DEEP: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_PRIORITY: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_CONTENT: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

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

    pub fn get_helper_text(&self, mode: &DirFilePriorityForm) -> &'static str {
        match mode {
            DirFilePriorityForm::Regex => HELP_REGEX,
            DirFilePriorityForm::Deep => HELP_DEEP,
            DirFilePriorityForm::Priority => HELP_PRIORITY,
            DirFilePriorityForm::Content => HELP_CONTENT,
            DirFilePriorityForm::Submit => HELP_SUBMIT,
        }
    }
}
