use crate::application::{ app_mode::{ AppMode, FileFilterForm }, app::App };

use super::file_filter_form_state::FileFilterFormState;

const HELP: &'static str = "| ESC~Exit |";

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

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
