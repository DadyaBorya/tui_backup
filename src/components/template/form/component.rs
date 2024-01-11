use std::path::PathBuf;

use crate::{
    application::{
        app::App,
        mode::{AppMode, TemplateForm},
    },
    components::{
        file::list::state::FileListState,
        popup::{
            confirm::component::ConfirmPopupComponent, message::component::MessagePopupComponent,
        },
        tab::component::TabComponent,
    },
    services::{file_service, map_template_service},
};

use super::state::TemplateFormState;

const HELP_NAME: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct TemplateFormComponent {
    pub state: TemplateFormState,
}

impl TemplateFormComponent {
    pub fn init() -> Self {
        TemplateFormComponent {
            state: TemplateFormState::init(),
        }
    }

    pub fn exit(app: &mut App) {
        ConfirmPopupComponent::show(
            app,
            "Confiramation".to_string(),
            "Do you want to exit?".to_string(),
            AppMode::FileList,
        );
    }

    pub fn next(app: &mut App, next: TemplateForm) {
        app.change_mode(AppMode::TemplateForm(next), app.state.mode.clone());
    }

    pub fn submit(app: &mut App) {
        let name = match app.components.template_form.state.validate() {
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

        app.components.template_form.state.clear();

        app.components.template_form.state.is_edit = false;

        app.components.file_list.state = FileListState::init().unwrap();

        let template_list = &mut app.components.template_list;

        template_list.state.renew();

        TabComponent::change_preview(app, 1);
    }

    pub fn get_helper_text(&self, mode: &TemplateForm) -> &'static str {
        match mode {
            TemplateForm::Name => HELP_NAME,
            TemplateForm::Submit => HELP_SUBMIT,
        }
    }
}
