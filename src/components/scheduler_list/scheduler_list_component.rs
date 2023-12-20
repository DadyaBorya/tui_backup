use crate::{
    models::config::Config,
    application::{ app::App, app_mode::AppMode },
    utils::table_util,
};

use super::scheduler_list_state::SchedulerListState;

const HELP: &'static str = "| ESC~Back | ↑ ↓ Move | d~Delete | | e~Edit |";

pub struct SchedulerListComponent {
    pub state: SchedulerListState,
}

impl SchedulerListComponent {
    pub fn init(config: &Config) -> Result<Self, std::io::Error> {
        Ok(SchedulerListComponent {
            state: SchedulerListState::init(config)?,
        })
    }

    pub fn move_up(&mut self) {
        table_util::move_up(&mut self.state.table_state, self.state.schedulers.len());
    }

    pub fn move_down(&mut self) {
        table_util::move_down(&mut self.state.table_state, self.state.schedulers.len());
    }

    pub fn exit(app: &mut App) {
        let state = &mut app.components.scheduler_list.state;
        state.table_state.select(None);
        app.change_mode(AppMode::Tab, AppMode::SchedulerList);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
