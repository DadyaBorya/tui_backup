use crate::application::{
    app::App,
    mode::{AppMode, Confirm},
};

use super::state::ConfirmPopupState;

const HELP_CANCEL: &'static str = "| ESC~Exit | TAB~Next | ENTER~Submit |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | BACKTAB~Prev | ENTER~Submit |";

pub struct ConfirmPopupComponent {
    pub state: ConfirmPopupState,
}

impl ConfirmPopupComponent {
    pub fn init() -> Self {
        ConfirmPopupComponent {
            state: ConfirmPopupState::init(),
        }
    }

    pub fn exit(app: &mut App) {
        let mode = app.components.confirm.state.prev_mode.clone();
        app.change_mode(mode, app.state.prev_mode.clone());
    }

    pub fn next(app: &mut App, mode: Confirm) {
        app.change_mode(AppMode::Confirm(mode), app.state.prev_mode.clone());
    }

    pub fn submit(app: &mut App) {
        let mode = app.components.confirm.state.next_mode.clone();
        let prev_mode = app.components.confirm.state.prev_mode.clone();

        match prev_mode {
            AppMode::TemplateForm(_) => {
                if !app.components.template_form.state.is_edit {
                    app.components.template_form.state.clear();
                }
            }
            AppMode::DirPriorityForm(_) => {
                app.components.dir_priority_form.state.clear();
            }
            AppMode::DirFilePriorityForm(_) => {
                app.components.dir_file_priority_form.state.clear();
            }
            AppMode::FilePriorityForm(_) => {
                app.components.file_priority_form.state.clear();
            }
            AppMode::FileFilterForm(_) => {
                app.components.file_filter_form.state.clear();
            }
            AppMode::SchedulerForm(_) => {
                app.components.scheduler_form.state.clear();
            }
            _ => {}
        }

        app.change_mode(mode, app.state.prev_mode.clone());
    }

    pub fn show(app: &mut App, title: String, message: String, mode: AppMode) {
        app.components
            .confirm
            .state
            .edit(title, message, app.state.mode.clone(), mode);
        app.change_mode(AppMode::Confirm(Confirm::Cancel), app.state.mode.clone());
    }

    pub fn get_helper_text(&self, mode: &Confirm) -> &'static str {
        match mode {
            Confirm::Cancel => HELP_CANCEL,
            Confirm::Submit => HELP_SUBMIT,
        }
    }
}
