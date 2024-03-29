use std::path::PathBuf;

use crate::{
    models::{ config::Config, dir_entry::DirEntry },
    application::{ app::App, mode::{ AppMode, SchedulerForm } },
    utils::list_utils,
    services::{ file_service, map_dir_entry_service, file_system_service },
    components::tab::component::TabComponent,
};

use super::state::TemplateListState;

const HELP: &'static str = "| ESC~Back | ↑ ↓ Move | d~Delete | | e~Edit | c~Create Scheduler |";

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

    pub fn edit(app: &mut App) {
        let template_list = &mut app.components.template_list;

        if let Some(template) = template_list.state.selected() {
            if
                let Ok(content) = file_service::read_file(
                    &PathBuf::from(
                        format!("{}/{}", &template_list.state.template_path.display(), template)
                    )
                )
            {
                if let Ok(entries) = map_dir_entry_service::template_to_dir_entry(content) {
                    let mut first_path = entries.first().unwrap().path.clone();
                    first_path.pop();

                    match first_path == entries.first().unwrap().path {
                        true => {
                            if first_path.to_str().map_or(false, |s| s.contains(":")) {
                                first_path = PathBuf::from("/");
                            }
                        }
                        _ => {}
                    }

                    let mut root = DirEntry::default();
                    root.path = PathBuf::from("/");
                    root.children = Some(file_service::root().unwrap());

                    file_system_service::add_vec_items(&mut root, entries);

                    let file_list = &mut app.components.file_list;

                    file_list.state.root = root;

                    let ancestors_vec: Vec<_> = first_path.ancestors().collect();
                    file_list.state.history = vec![0; ancestors_vec.len()];

                    file_list.state.current_path = first_path;
                    file_list.state.root.sort_children();
                    file_list.state.set_rows();

                    app.components.template_list.state.list_state.select(None);
                    app.components.template_form.state.name = template;
                    app.components.template_form.state.is_edit = true;

                    TabComponent::change_preview(app, 0);
                }
            }
        }
    }

    pub fn exit(app: &mut App) {
        let state = &mut app.components.template_list.state;
        state.list_state.select(None);
        app.change_mode(AppMode::Tab, AppMode::TemplateList);
    }

    pub fn delete(&mut self) {
        if let Some(template) = self.state.selected() {
            let mut path = self.state.template_path.clone();
            path.push(template);
            if let Ok(_) = file_service::delete_file(&path) {
                self.state.renew();
                self.move_up();
            }
        }
    }

    pub fn create_scheduler(app: &mut App) {
        let template_state = &mut app.components.template_list.state;

        if let Some(name) = template_state.selected() {
            let scheduler_state = &mut app.components.scheduler_form.state;
            scheduler_state.root = format!("{}/{}", &app.config.paths.templates, name);
            app.change_mode(
                AppMode::SchedulerForm(SchedulerForm::Name),
                app.state.prev_mode.clone()
            )
        }
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
