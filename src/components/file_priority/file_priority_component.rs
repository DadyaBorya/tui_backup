use crate::{
    application::{ app::App, app_mode::{ AppMode, FilePriorityForm } },
    utils::list_utils,
    components::message_popup::message_popup_components::MessagePopupComponent,
};

use super::file_priority_state::FilePriorityState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | n~New | d~Delete | e~Edit |";

pub struct FilePriorityComponent {
    pub state: FilePriorityState,
}

impl FilePriorityComponent {
    pub fn init() -> Self {
        FilePriorityComponent {
            state: FilePriorityState::init(),
        }
    }

    pub fn move_up(&mut self) {
        list_utils::move_up(&mut self.state.list_state, self.state.rules.len());
    }

    pub fn move_down(&mut self) {
        list_utils::move_down(&mut self.state.list_state, self.state.rules.len());
    }

    pub fn delete(app: &mut App) {
        if let Some(entry) = app.components.file_list.state.get_selected_entry() {
            if let Some(index) = app.components.file_priority.state.list_state.selected() {
                let priority_root = entry.entry_file_priority.as_ref().unwrap()[index].root.clone();

                if priority_root != entry.path() {
                    MessagePopupComponent::show(
                        app,
                        "Can't delete root priority".to_string(),
                        format!("Root priority is {}", priority_root)
                    );
                    return;
                }

                entry.entry_file_priority.as_mut().unwrap().remove(index);

                if entry.entry_file_priority.as_ref().unwrap().is_empty() {
                    entry.entry_file_priority = None;
                }

                app.components.file_priority.state.rules.remove(index);
                app.components.file_priority.move_up();
            }
        }
    }

    pub fn edit(app: &mut App) {
        if let Some(index) = app.components.file_priority.state.list_state.selected() {
            let priority = app.components.file_priority.state.rules[index].clone();

            let entry = app.components.file_list.state.get_selected_entry().unwrap();

            if priority.root != entry.path() {
                MessagePopupComponent::show(
                    app,
                    "Can't edit root priority".to_string(),
                    format!("Root priority is {}", priority.root)
                );
                return;
            }

            let state = &mut app.components.file_priority_form.state;
            state.content = priority.content;
            state.priority = priority.priority.to_string();
            app.components.file_priority.state.is_edit = true;
            app.change_mode(
                AppMode::FilePriorityForm(FilePriorityForm::Priority),
                app.state.prev_mode.clone()
            );
        }
    }

    pub fn exit(app: &mut App) {
        let file_priority = &mut app.components.file_priority;
        file_priority.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::FilePriority);
    }

    pub fn new_rule(app: &mut App) {
        app.change_mode(
            AppMode::FilePriorityForm(FilePriorityForm::Priority),
            AppMode::FilePriority
        );
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
