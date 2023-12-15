use crate::{
    application::{ app::App, app_mode::{ FilePriorityForm, AppMode } },
    models::entry_file_priority::EntryFilePriority,
    components::message_popup::message_popup_components::MessagePopupComponent,
};

use super::file_priority_form_state::FilePriorityFormState;

const HELP_CONTENT: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_PRIORITY: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct FilePriorityFormComponent {
    pub state: FilePriorityFormState,
}

impl FilePriorityFormComponent {
    pub fn init() -> Self {
        FilePriorityFormComponent {
            state: FilePriorityFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: FilePriorityForm) {
        app.components.file_priority_form.state.clear();
        app.change_mode(AppMode::FilePriority, AppMode::FilePriorityForm(prev_mode));
    }

    pub fn create(app: &mut App) -> Option<EntryFilePriority> {
        let state = &app.components.file_priority_form.state;

        let validate = state.validate();

        match validate {
            Ok(value) => { Some(value) }
            Err(errors) => {
                MessagePopupComponent::show_vec(
                    app,
                    errors,
                    AppMode::FilePriorityForm(FilePriorityForm::Submit)
                );
                None
            }
        }
    }

    pub fn add(app: &mut App) {
        let filter = match FilePriorityFormComponent::create(app) {
            Some(value) => value,
            None => {
                return;
            }
        };

        let entry = app.components.file_list.state.get_selected_entry().unwrap();

        match app.components.file_priority.state.is_edit {
            true => {
                if let Some(rules) = entry.entry_file_priority.as_mut() {
                    let index = app.components.file_priority.state.list_state.selected().unwrap();
                    rules[index] = filter;
                }

                app.components.file_priority.state.is_edit = false;
            }
            false => entry.entry_file_priority.get_or_insert(Vec::new()).push(filter),
        }

        app.components.file_priority_form.state.clear();
        app.change_mode(AppMode::FilePriority, app.state.mode.clone());
    }

    pub fn next(app: &mut App, next: FilePriorityForm, prev_mode: FilePriorityForm) {
        app.change_mode(AppMode::FilePriorityForm(next), AppMode::FilePriorityForm(prev_mode));
    }

    pub fn get_help_text(&self, mode: &FilePriorityForm) -> &'static str {
        match mode {
            FilePriorityForm::Priority => HELP_PRIORITY,
            FilePriorityForm::Content => HELP_CONTENT,
            FilePriorityForm::Submit => HELP_SUBMIT,
        }
    }
}