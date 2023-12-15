use crate::{
    application::{ app::App, app_mode::{ DirFilterForm, AppMode } },
    models::entry_dir_filter::EntryDirFilter,
    components::message_popup::message_popup_components::MessagePopupComponent,
};

use super::dir_filter_form_state::DirFilterFormState;

const HELP_REGEX: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_DEEP: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct DirFilterFormComponent {
    pub state: DirFilterFormState,
}

impl DirFilterFormComponent {
    pub fn init() -> Self {
        DirFilterFormComponent {
            state: DirFilterFormState::init(),
        }
    }

    pub fn exit(app: &mut App, prev_mode: DirFilterForm) {
        app.components.dir_filter_form.state.clear();
        app.change_mode(AppMode::DirFilter, AppMode::DirFilterForm(prev_mode));
    }

    pub fn next(app: &mut App, next: DirFilterForm, prev_mode: DirFilterForm) {
        app.change_mode(AppMode::DirFilterForm(next), AppMode::DirFilterForm(prev_mode));
    }

    pub fn create(app: &mut App) -> Option<EntryDirFilter> {
        let state = &app.components.dir_filter_form.state;

        let validate = state.validate();

        match validate {
            Ok(value) => { Some(value) }
            Err(errors) => {
                MessagePopupComponent::show_vec(
                    app,
                    errors,
                    AppMode::DirFilterForm(DirFilterForm::Submit)
                );
                None
            }
        }
    }

    pub fn add(app: &mut App) {
        let mut filter = match DirFilterFormComponent::create(app) {
            Some(value) => value,
            None => {
                return;
            }
        };

        let entry = app.components.file_list.state.get_selected_entry().unwrap();
        filter.root = entry.path();

        match app.components.dir_filter.state.is_edit {
            true => {
                if let Some(rules) = entry.entry_dir_filter.as_mut() {
                    let index = app.components.dir_filter.state.list_state.selected().unwrap();
                    rules[index] = filter;
                }

                app.components.dir_filter.state.is_edit = false;
            }
            false => entry.entry_dir_filter.get_or_insert(Vec::new()).push(filter),
        }

        app.components.dir_filter_form.state.clear();
        app.change_mode(AppMode::DirFilter, app.state.mode.clone());
    }

    pub fn get_helper_text(&self, mode: &DirFilterForm) -> &'static str {
        match mode {
            DirFilterForm::Regex => HELP_REGEX,
            DirFilterForm::Deep => HELP_DEEP,
            DirFilterForm::Submit => HELP_SUBMIT,
        }
    }
}
