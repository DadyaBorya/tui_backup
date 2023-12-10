use crate::application::{ app::App, app_mode::AppMode };

use super::file_filter_state::FileFilterState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | ]~Next |";

pub struct FileFilterComponent {
    pub state: FileFilterState,
}

impl FileFilterComponent {
    pub fn init() -> Self {
        FileFilterComponent {
            state: FileFilterState::init(),
        }
    }

    pub fn exit(app: &mut App) {
        let file_filter = &mut app.components.file_filter;
        file_filter.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::FileFilter);
    }

    pub fn next_component(app: &mut App) {
        let file_filter = &mut app.components.file_filter;
        file_filter.state.list_state.select(None);
        app.change_mode(AppMode::DirFilter, AppMode::FileList);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
