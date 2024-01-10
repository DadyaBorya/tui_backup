use crate::{ utils::tab_util, application::{ app::App, mode::AppMode } };

use super::state::TabState;

const HELP: &'static str = "| ← Prev | → Next | ENTER~Select | q~Exit |";

pub struct TabComponent {
    pub state: TabState,
}

impl TabComponent {
    pub fn init() -> Self {
        TabComponent { state: TabState::init() }
    }

    pub fn next(&mut self) {
        let index = tab_util::next(self.state.index, self.state.headers.len());
        self.state.index = index;
    }

    pub fn previous(&mut self) {
        let index = tab_util::previous(self.state.index, self.state.headers.len());
        self.state.index = index;
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }

    pub fn change_preview(app: &mut App, index: usize) {
        app.components.tabs.state.index = index;
        TabComponent::select_tab(app);
    }

    pub fn select_tab(app: &mut App) {
        let index = app.components.tabs.state.index;
        match index {
            0 => {
                app.components.file_list.state.init_index_table();
                app.change_mode(AppMode::FileList, AppMode::Tab)
            }
            1 => {
                app.components.template_list.state.init_index_table();
                app.change_mode(AppMode::TemplateList, AppMode::Tab)
            }
            2 => {
                app.components.scheduler_list.state.init_index_table();
                app.change_mode(AppMode::SchedulerList, AppMode::Tab)
            }
            _ => {}
        }
    }
}
