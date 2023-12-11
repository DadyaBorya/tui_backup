use crate::application::{ app_mode::{ AppMode, FileFilterForm }, app::App };

use super::file_filter_form_state::FileFilterFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_DEEP: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_CONTENT: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct FileFilterFormComponent {
    pub state: FileFilterFormState,
}

impl FileFilterFormComponent {
    pub fn init() -> Self {
        FileFilterFormComponent {
            state: FileFilterFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: FileFilterForm) {
        app.change_mode(AppMode::FileFilter, AppMode::FileFilterForm(prev_mode));
    }

    pub fn next(app: &mut App, next: FileFilterForm, prev_mode: FileFilterForm) {
        app.change_mode(AppMode::FileFilterForm(next), AppMode::FileFilterForm(prev_mode));
    }

    pub fn get_helper_text(&self, mode: &FileFilterForm) -> &'static str {
        match mode {
            FileFilterForm::Regex => HELP_REGEX,
            FileFilterForm::Content => HELP_CONTENT,
            FileFilterForm::Deep => HELP_DEEP,
            FileFilterForm::Submit => HELP_SUBMIT,
        }
    }
}
