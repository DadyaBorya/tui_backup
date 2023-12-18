use crate::{ models::config::Config, application::{ app::App, app_mode::AppMode } };

use super::template_list_state::TemplateListState;

const HELP: &'static str = "";

pub struct TemplateListComponent {
    pub state: TemplateListState,
}

impl TemplateListComponent {
    pub fn init(config: &Config) -> Result<Self, std::io::Error> {
        Ok(TemplateListComponent {
            state: TemplateListState::init(config)?,
        })
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
