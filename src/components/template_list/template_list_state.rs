use std::path::PathBuf;

use tui::widgets::ListState;

use crate::{
    models::config::Config,
    services::{ file_service, map_dir_entry_service },
    utils::list_utils,
};

#[derive(Default)]
pub struct TemplateListState {
    pub list_state: ListState,
    pub templates: Vec<String>,
    pub template_path: PathBuf,
}

impl TemplateListState {
    pub fn init(config: &Config) -> Result<Self, std::io::Error> {
        let mut state = TemplateListState::default();
        state.template_path = PathBuf::from(config.paths.templates.clone());
        state.renew();
        Ok(state)
    }

    pub fn init_index_table(&mut self) {
        let len = self.templates.len();
        list_utils::init_index_table(&mut self.list_state, len)
    }

    pub fn selected(&self) -> Option<String> {
        if let Some(index) = self.list_state.selected() {
            let template = self.templates[index].clone();
            return Some(template);
        }

        None
    }

    pub fn renew(&mut self) {
        self.templates.clear();

        if let Ok(entries) = file_service::entries(&self.template_path) {
            for entry in entries.iter().filter(|entry| !entry.is_dir()) {
                if let Ok(content) = file_service::read_file(&entry.path) {
                    if let Ok(_) = map_dir_entry_service::template_to_dir_entry(content) {
                        self.templates.push(entry.file_name());
                    }
                }
            }
        }
    }

    pub fn rows(&self) -> Vec<String> {
        self.templates.clone()
    }
}
