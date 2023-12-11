use crate::application::{ app::App, app_mode::{ DirFilterForm, AppMode } };

use super::dir_filter_form_state::DirFilterFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_DEEP: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct DirFilterFormComponent {
    pub state: DirFilterFormState,
}

impl DirFilterFormComponent {
    pub fn init() -> Self {
        DirFilterFormComponent {
            state: DirFilterFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: DirFilterForm) {
        app.change_mode(AppMode::DirFilter, AppMode::DirFilterForm(prev_mode));
    }

    pub fn next(app: &mut App, next: DirFilterForm, prev_mode: DirFilterForm) {
        app.change_mode(AppMode::DirFilterForm(next), AppMode::DirFilterForm(prev_mode));
    }

    pub fn get_helper_text(&self, mode: &DirFilterForm) -> &'static str {
        match mode {
            DirFilterForm::Regex => HELP_REGEX,
            DirFilterForm::Deep => HELP_DEEP,
            DirFilterForm::Submit => HELP_SUBMIT,
        }
    }
}
