use crate::{
    application::{ app::App, app_mode::{ DirFilePriorityForm, AppMode } },
    models::entry_dir_file_priority::EntryDirFilePriority,
    components::message_popup::message_popup_components::MessagePopupComponent,
};

use super::dir_file_priority_form_state::DirFilePriorityFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_DEEP: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_PRIORITY: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_CONTENT: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct DirFilePriorityFormComponent {
    pub state: DirFilePriorityFormState,
}

impl DirFilePriorityFormComponent {
    pub fn init() -> Self {
        DirFilePriorityFormComponent {
            state: DirFilePriorityFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: DirFilePriorityForm) {
        app.components.dir_file_priority_form.state.clear();
        app.change_mode(AppMode::DirFilePriority, AppMode::DirFilePriorityForm(prev_mode));
    }

    pub fn create(app: &mut App) -> Option<EntryDirFilePriority> {
        let state = &app.components.dir_file_priority_form.state;

        let validate = state.validate();

        match validate {
            Ok(value) => { Some(value) }
            Err(errors) => {
                MessagePopupComponent::show_vec(
                    app,
                    errors,
                    AppMode::DirFilePriorityForm(DirFilePriorityForm::Submit)
                );
                None
            }
        }
    }

    pub fn add(app: &mut App) {
        let mut filter = match DirFilePriorityFormComponent::create(app) {
            Some(value) => value,
            None => {
                return;
            }
        };

        let entry = app.components.file_list.state.get_selected_entry().unwrap();
        filter.root = Some(entry.path.as_path().display().to_string());

        match app.components.dir_file_priority.state.is_edit {
            true => {
                if let Some(rules) = entry.entry_dir_file_priority.as_mut() {
                    let index = app.components.dir_file_priority.state.list_state
                        .selected()
                        .unwrap();
                    rules[index] = filter;
                }

                app.components.dir_filter.state.is_edit = false;
            }
            false => entry.entry_dir_file_priority.get_or_insert(Vec::new()).push(filter),
        }

        app.components.dir_file_priority_form.state.clear();
        app.change_mode(AppMode::DirFilePriority, app.state.mode.clone());
    }

    pub fn next(app: &mut App, next: DirFilePriorityForm, prev_mode: DirFilePriorityForm) {
        app.change_mode(
            AppMode::DirFilePriorityForm(next),
            AppMode::DirFilePriorityForm(prev_mode)
        );
    }

    pub fn get_helper_text(&self, mode: &DirFilePriorityForm) -> &'static str {
        match mode {
            DirFilePriorityForm::Regex => HELP_REGEX,
            DirFilePriorityForm::Deep => HELP_DEEP,
            DirFilePriorityForm::Priority => HELP_PRIORITY,
            DirFilePriorityForm::Content => HELP_CONTENT,
            DirFilePriorityForm::Submit => HELP_SUBMIT,
        }
    }
}
