use crate::application::{ app_mode::{ SchedulerForm, AppMode }, app::App };

use super::scheduler_form_state::SchedulerFormState;

const HELP: &'static str = "";

pub struct SchedulerFormComponent {
    pub state: SchedulerFormState,
}

impl SchedulerFormComponent {
    pub fn init() -> Self {
        SchedulerFormComponent {
            state: SchedulerFormState::init(),
        }
    }

    pub fn exit(app: &mut App) {
        app.components.scheduler_form.state.clear();
        app.change_mode(AppMode::TemplateList, app.state.mode.clone());
    }

    pub fn next(app: &mut App, mode: SchedulerForm) {
        app.change_mode(AppMode::SchedulerForm(mode), app.state.mode.clone());
    }

    pub fn get_helper_text(&self, mode: &SchedulerForm) -> &'static str {
        match mode {
            _ => HELP,
        }
    }
}
