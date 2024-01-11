use crate::{
    application::{
        app::App,
        mode::{AppMode, DirPriorityForm},
    },
    components::popup::{
        confirm::component::ConfirmPopupComponent, message::component::MessagePopupComponent,
    },
    models::entry_dir_priority::EntryDirPriority,
};

use super::state::DirPriorityFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_DEEP: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_PRIORITY: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct DirPriorityFormComponent {
    pub state: DirPriorityFormState,
}

impl DirPriorityFormComponent {
    pub fn init() -> Self {
        DirPriorityFormComponent {
            state: DirPriorityFormState::init(),
        }
    }

    pub fn exit(app: &mut App) {
        ConfirmPopupComponent::show(
            app,
            "Confiramation".to_string(),
            "Do you want to exit?".to_string(),
            AppMode::DirPriority,
        );
    }

    pub fn create(app: &mut App) -> Option<EntryDirPriority> {
        let state = &app.components.dir_priority_form.state;

        let validate = state.validate();

        match validate {
            Ok(value) => Some(value),
            Err(errors) => {
                MessagePopupComponent::show_vec(
                    app,
                    errors,
                    AppMode::DirPriorityForm(DirPriorityForm::Submit),
                );
                None
            }
        }
    }

    pub fn add(app: &mut App) {
        let mut filter = match DirPriorityFormComponent::create(app) {
            Some(value) => value,
            None => {
                return;
            }
        };

        let entry = app.components.file_list.state.get_selected_entry().unwrap();
        filter.root = entry.path();

        match app.components.dir_priority.state.is_edit {
            true => {
                if let Some(rules) = entry.entry_dir_priority.as_mut() {
                    let index = app
                        .components
                        .dir_priority
                        .state
                        .list_state
                        .selected()
                        .unwrap();
                    rules[index] = filter;
                }

                app.components.dir_priority.state.is_edit = false;
            }
            false => entry
                .entry_dir_priority
                .get_or_insert(Vec::new())
                .push(filter),
        }

        app.components.dir_priority_form.state.clear();
        app.change_mode(AppMode::DirPriority, app.state.mode.clone());
    }

    pub fn next(app: &mut App, next: DirPriorityForm) {
        app.change_mode(AppMode::DirPriorityForm(next), app.state.mode.clone());
    }

    pub fn get_help_text(&self, mode: &DirPriorityForm) -> &'static str {
        match mode {
            DirPriorityForm::Regex => HELP_REGEX,
            DirPriorityForm::Deep => HELP_DEEP,
            DirPriorityForm::Priority => HELP_PRIORITY,
            DirPriorityForm::Submit => HELP_SUBMIT,
        }
    }
}
