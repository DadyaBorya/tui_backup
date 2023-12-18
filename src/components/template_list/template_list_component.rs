use crate::{
    models::config::Config,
    application::{ app::App, app_mode::AppMode },
    utils::list_utils,
};

use super::template_list_state::TemplateListState;

const HELP: &'static str =
    "| ESC~Back | ↑ ↓ Move | d~Delete | | e~Edit |";

pub struct TemplateListComponent {
    pub state: TemplateListState,
}

impl TemplateListComponent {
    pub fn init(config: &Config) -> Result<Self, std::io::Error> {
        Ok(TemplateListComponent {
            state: TemplateListState::init(config)?,
        })
    }

    pub fn move_up(&mut self) {
        list_utils::move_up(&mut self.state.list_state, self.state.templates.len());
    }

    pub fn move_down(&mut self) {
        list_utils::move_down(&mut self.state.list_state, self.state.templates.len());
    }

    pub fn exit(app: &mut App) {
        let state = &mut app.components.template_list.state;
        state.list_state.select(None);
        app.change_mode(AppMode::Tab, AppMode::TemplateList);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
