use crate::{
    application::{ app::App, app_mode::{ AppMode, DirPriorityForm } },
    utils::list_utils,
    components::message_popup::message_popup_components::MessagePopupComponent,
};

use super::dir_priority_state::DirPriorityState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | [~Prev | n~New | d~Delete | e~Edit |";

pub struct DirPriorityComponent {
    pub state: DirPriorityState,
}

impl DirPriorityComponent {
    pub fn init() -> Self {
        DirPriorityComponent {
            state: DirPriorityState::init(),
        }
    }

    pub fn delete(app: &mut App) {
        if let Some(entry) = app.components.file_list.state.get_selected_entry() {
            if let Some(index) = app.components.dir_priority.state.list_state.selected() {
                let priority_root = entry.entry_dir_priority.as_ref().unwrap()[index].root.clone();

                if priority_root != entry.path() {
                    MessagePopupComponent::show(
                        app,
                        "Can't delete root priority".to_string(),
                        format!("Root priority is {}", priority_root)
                    );
                    return;
                }
                entry.entry_dir_priority.as_mut().unwrap().remove(index);

                if entry.entry_dir_priority.as_ref().unwrap().is_empty() {
                    entry.entry_dir_priority = None;
                }

                app.components.dir_priority.state.rules.remove(index);
                app.components.dir_priority.move_up();
            }
        }
    }

    pub fn exit(app: &mut App) {
        let dir_priority = &mut app.components.dir_priority;
        dir_priority.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::DirPriority);
    }

    pub fn edit(app: &mut App) {
        if let Some(index) = app.components.dir_priority.state.list_state.selected() {
            let priority = app.components.dir_priority.state.rules[index].clone();

            let entry = app.components.file_list.state.get_selected_entry().unwrap();

            if priority.root != entry.path() {
                MessagePopupComponent::show(
                    app,
                    "Can't delete root priority".to_string(),
                    format!("Root priority is {}", priority.root)
                );
                return;
            }
            let state = &mut app.components.dir_priority_form.state;
            state.regex = priority.regex;
            state.deep = priority.deep.to_string();
            state.priority = priority.priority.to_string();
            app.components.dir_priority.state.is_edit = true;
            app.change_mode(
                AppMode::DirPriorityForm(DirPriorityForm::Regex),
                app.state.prev_mode.clone()
            );
        }
    }

    pub fn move_up(&mut self) {
        list_utils::move_up(&mut self.state.list_state, self.state.rules.len());
    }

    pub fn move_down(&mut self) {
        list_utils::move_down(&mut self.state.list_state, self.state.rules.len());
    }

    pub fn prev_component(app: &mut App) {
        let dir_priority = &mut app.components.dir_priority;
        dir_priority.state.list_state.select(None);
        app.change_mode(AppMode::DirFilePriority, AppMode::DirPriority);
    }

    pub fn new_rule(app: &mut App) {
        app.change_mode(AppMode::DirPriorityForm(DirPriorityForm::Regex), AppMode::DirPriority);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
