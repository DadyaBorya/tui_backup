use crate::application::{ app::App, app_mode::{AppMode, DirFilterForm} };

use super::dir_filter_state::DirFilterState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | [~Prev |";

pub struct DirFilterComponent {
    pub state: DirFilterState,
}

impl DirFilterComponent {
    pub fn init() -> Self {
        DirFilterComponent {
            state: DirFilterState::init(),
        }
    }

    pub fn exit(app: &mut App) {
        let dir_filter = &mut app.components.dir_filter;
        dir_filter.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::DirFilter);
    }

    pub fn prev_component(app: &mut App) {
        let dir_filter = &mut app.components.dir_filter;
        dir_filter.state.list_state.select(None);
        app.change_mode(AppMode::FileFilter, AppMode::DirFilter);
    }

    pub fn new_rule(app: &mut App) {
        app.change_mode(AppMode::DirFilterForm(DirFilterForm::Regex), AppMode::DirFilter);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
