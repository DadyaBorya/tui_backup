use std::path::PathBuf;

use crate::{
    application::{ app_mode::{ CreateTemplateForm, AppMode }, app::App },
    services::{ map_template_service, file_service },
    components::{
        message_popup::message_popup_components::MessagePopupComponent,
        tab::tab_component::TabComponent,
        file_list::file_list_state::FileListState,
    },
};

use super::create_template_form_state::CreateTemplateFormState;

const HELP_NAME: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct CreateTemplateFormComponent {
    pub state: CreateTemplateFormState,
}

impl CreateTemplateFormComponent {
    pub fn init() -> Self {
        CreateTemplateFormComponent {
            state: CreateTemplateFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: CreateTemplateForm) {
        app.components.create_template_form.state.clear();
        app.change_mode(AppMode::FileList, AppMode::CreateTemplateForm(prev_mode));
    }

    pub fn next(app: &mut App, next: CreateTemplateForm, prev_mode: CreateTemplateForm) {
        app.change_mode(AppMode::CreateTemplateForm(next), AppMode::CreateTemplateForm(prev_mode));
    }

    pub fn submit(app: &mut App) {
        let name = match app.components.create_template_form.state.validate() {
            Ok(name) => name,
            Err(errors) => {
                MessagePopupComponent::show_vec(app, errors, app.state.mode.clone());
                return;
            }
        };

        let entry = &mut app.components.file_list.state.root;

        let content = map_template_service::dir_entry_to_template(entry);

        if content.is_empty() {
            MessagePopupComponent::show(app, "Error".to_string(), "Template is empty".to_string());
            return;
        }

        let dir_path = &app.config.paths.templates;

        let _ = file_service::create_dir(&PathBuf::from(dir_path));

        let path = format!("{}/{}", dir_path, name);

        let _ = file_service::save(&PathBuf::from(path), content);

        app.components.create_template_form.state.clear();

        app.components.file_list.state = FileListState::init().unwrap();

        let template_list = &mut app.components.template_list;

        template_list.state.renew();

        TabComponent::change_preview(app, 1);
    }

    pub fn get_helper_text(&self, mode: &CreateTemplateForm) -> &'static str {
        match mode {
            CreateTemplateForm::Name => HELP_NAME,
            CreateTemplateForm::Submit => HELP_SUBMIT,
        }
    }
}
