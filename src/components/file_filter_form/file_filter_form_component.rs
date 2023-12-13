use crate::{
    application::{ app_mode::{ AppMode, FileFilterForm }, app::App },
    components::message_popup::message_popup_components::MessagePopupComponent,
    models::entry_file_filter::EntryFileFilter,
};

use super::file_filter_form_state::FileFilterFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_DEEP: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_CONTENT: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct FileFilterFormComponent {
    pub state: FileFilterFormState,
}

impl FileFilterFormComponent {
    pub fn init() -> Self {
        FileFilterFormComponent {
            state: FileFilterFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: FileFilterForm) {
        app.components.file_filter_form.state.clear();
        app.change_mode(AppMode::FileFilter, AppMode::FileFilterForm(prev_mode));
    }

    pub fn next(app: &mut App, next: FileFilterForm, prev_mode: FileFilterForm) {
        app.change_mode(AppMode::FileFilterForm(next), AppMode::FileFilterForm(prev_mode));
    }

    pub fn create(app: &mut App) -> Option<EntryFileFilter> {
        let state = &app.components.file_filter_form.state;

        let validate = state.validate();

        match validate {
            Ok(value) => { Some(value) }
            Err(errors) => {
                MessagePopupComponent::show_vec(
                    app,
                    errors,
                    AppMode::FileFilterForm(FileFilterForm::Submit)
                );
                None
            }
        }
    }

    pub fn add(app: &mut App) {
        let mut filter = match FileFilterFormComponent::create(app) {
            Some(value) => value,
            None => {
                return;
            }
        };

        let entry = app.components.file_list.state.get_selected_entry().unwrap();
        filter.root = Some(entry.path.as_path().display().to_string());

        match app.components.file_filter.state.is_edit {
            true => {
                if let Some(rules) = entry.entry_file_filter.as_mut() {
                    let index = app.components.file_filter.state.list_state.selected().unwrap();
                    rules[index] = filter;
                }

                app.components.file_filter.state.is_edit = false;
            }
            false => entry.entry_file_filter.get_or_insert(Vec::new()).push(filter),
        }

        app.components.file_filter_form.state.clear();
        app.change_mode(AppMode::FileFilter, app.state.mode.clone());
    }

    pub fn get_helper_text(&self, mode: &FileFilterForm) -> &'static str {
        match mode {
            FileFilterForm::Regex => HELP_REGEX,
            FileFilterForm::Content => HELP_CONTENT,
            FileFilterForm::Deep => HELP_DEEP,
            FileFilterForm::Submit => HELP_SUBMIT,
        }
    }
}
