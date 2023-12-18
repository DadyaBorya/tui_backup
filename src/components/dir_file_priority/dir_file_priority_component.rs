use crate::{
    application::{ app::App, app_mode::{ AppMode, DirFilePriorityForm } },
    utils::list_utils,
    components::message_popup::message_popup_components::MessagePopupComponent,
};

use super::dir_file_priority_state::DirFilePriorityState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | ]~Next | n~New | d~Delete | e~Edit |";

pub struct DirFilePriorityComponent {
    pub state: DirFilePriorityState,
}

impl DirFilePriorityComponent {
    pub fn init() -> Self {
        DirFilePriorityComponent {
            state: DirFilePriorityState::init(),
        }
    }

    pub fn exit(app: &mut App) {
        let dir_file_priority = &mut app.components.dir_file_priority;
        dir_file_priority.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::DirFilePriority);
    }

    pub fn next_component(app: &mut App) {
        let dir_priority = &mut app.components.dir_priority;
        dir_priority.state.init_index_table();
        let dir_file_priority = &mut app.components.dir_file_priority;
        dir_file_priority.state.list_state.select(None);
        app.change_mode(AppMode::DirPriority, AppMode::DirFilePriority);
    }

    pub fn move_up(&mut self) {
        list_utils::move_up(&mut self.state.list_state, self.state.rules.len());
    }

    pub fn move_down(&mut self) {
        list_utils::move_down(&mut self.state.list_state, self.state.rules.len());
    }

    pub fn delete(app: &mut App) {
        if let Some(entry) = app.components.file_list.state.get_selected_entry() {
            if let Some(index) = app.components.dir_file_priority.state.list_state.selected() {
                let priority_root = entry.entry_dir_file_priority
                    .as_ref()
                    .unwrap()
                    [index].root.clone();

                if priority_root != entry.path() {
                    MessagePopupComponent::show(
                        app,
                        "Can't delete root priority".to_string(),
                        format!("Root priority is {}", priority_root)
                    );
                    return;
                }

                entry.entry_dir_file_priority.as_mut().unwrap().remove(index);

                if entry.entry_dir_file_priority.as_ref().unwrap().is_empty() {
                    entry.entry_dir_file_priority = None;
                }

                app.components.dir_file_priority.state.rules.remove(index);
                app.components.dir_file_priority.move_up();
            }
        }
    }

    pub fn edit(app: &mut App) {
        if let Some(index) = app.components.dir_file_priority.state.list_state.selected() {
            let priority = app.components.dir_file_priority.state.rules[index].clone();

            let entry = app.components.file_list.state.get_selected_entry().unwrap();

            if priority.root != entry.path() {
                MessagePopupComponent::show(
                    app,
                    "Can't edit root priority".to_string(),
                    format!("Root priority is {}", priority.root)
                );
                return;
            }
            let state = &mut app.components.dir_file_priority_form.state;
            state.regex = priority.regex;
            state.content = priority.content;
            state.deep = priority.deep.to_string();
            state.priority = priority.priority.to_string();
            app.components.dir_file_priority.state.is_edit = true;
            app.change_mode(
                AppMode::DirFilePriorityForm(DirFilePriorityForm::Regex),
                app.state.prev_mode.clone()
            );
        }
    }

    pub fn new_rule(app: &mut App) {
        app.change_mode(
            AppMode::DirFilePriorityForm(DirFilePriorityForm::Regex),
            AppMode::DirFilePriority
        );
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
