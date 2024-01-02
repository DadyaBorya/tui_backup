use std::path::PathBuf;

use tui::{ widgets::TableState, style::Color };

use crate::{
    utils::table_util,
    services::file_service,
    models::{ scheduler::Scheduler, config::Config },
};

#[derive(Default)]
pub struct SchedulerListState {
    pub table_state: TableState,
    pub scheduler_path: PathBuf,
    pub schedulers: Vec<(Vec<String>, Color)>,
}

impl SchedulerListState {
    pub fn init(config: &Config) -> Result<Self, std::io::Error> {
        let mut state = SchedulerListState::default();
        state.scheduler_path = PathBuf::from(config.paths.schedulers.clone());
        state.renew();
        Ok(state)
    }

    pub fn init_index_table(&mut self) {
        let len = self.schedulers.len();
        table_util::init_index_table(&mut self.table_state, len)
    }

    pub fn selected(&self) -> Option<String> {
        if let Some(index) = self.table_state.selected() {
            return Some(format!("{}/{}.json", self.scheduler_path.display(), self.schedulers[index].0[0]));
        }

        None
    }

    pub fn renew(&mut self) {
        self.schedulers.clear();

        if let Ok(entries) = file_service::entries(&self.scheduler_path) {
            for entry in entries.iter().filter(|entry| !entry.is_dir()) {
                if let Ok(content) = file_service::read_file(&entry.path) {
                    if let Ok(scheduler) = serde_json::from_str::<Scheduler>(&content) {
                        self.schedulers.push(scheduler.row());
                    }
                }
            }
        }
    }

    pub fn rows(&self) -> Vec<(Vec<String>, Color)> {
        self.schedulers.clone()
    }
}
