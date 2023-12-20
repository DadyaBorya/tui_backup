use crate::{
    application::{ app_mode::{ SchedulerForm, AppMode }, app::App },
    utils::list_utils,
    models::scheduler::Scheduler,
    components::message_popup::message_popup_components::MessagePopupComponent,
};

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

    pub fn create(app: &mut App) -> Option<Scheduler> {
        let state = &app.components.scheduler_form.state;

        let validate = state.validate();

        match validate {
            Ok(value) => { Some(value) }
            Err(errors) => {
                MessagePopupComponent::show_vec(
                    app,
                    errors,
                    AppMode::SchedulerForm(SchedulerForm::Submit)
                );
                None
            }
        }
    }

    pub fn add(app: &mut App) {
        let scheduler = match SchedulerFormComponent::create(app) {
            Some(value) => value,
            None => {
                return;
            }
        };
    }

    pub fn exit(app: &mut App) {
        app.components.scheduler_form.state.clear();
        app.change_mode(AppMode::TemplateList, app.state.mode.clone());
    }

    pub fn paste_current_cron(&mut self) {
        if let Some((_, v)) = self.state.selected_cron() {
            self.state.cron = v;
        }
    }

    pub fn select_cloud(&mut self) {
        if let Some((k, _)) = self.state.selected_cloud() {
            k.1 = !k.1;
        }
    }

    pub fn select_protocol(&mut self) {
        if let Some(protocol) = self.state.selected_protocols() {
            protocol.1 = !protocol.1;
        }
    }

    pub fn move_up(&mut self, mode: SchedulerForm) {
        match mode {
            SchedulerForm::Cron =>
                list_utils::move_up(
                    &mut self.state.cron_list_state,
                    self.state.cron_templates.len()
                ),
            SchedulerForm::Cloud =>
                list_utils::move_up(
                    &mut self.state.cloud_list_state,
                    self.state.clouds_protocols.len()
                ),
            SchedulerForm::Protocol => {
                let len = self.state.protocols().len();
                list_utils::move_up(&mut self.state.protocol_list_state, len);
            }
            _ => {}
        }
    }

    pub fn move_down(&mut self, mode: SchedulerForm) {
        match mode {
            SchedulerForm::Cron =>
                list_utils::move_down(
                    &mut self.state.cron_list_state,
                    self.state.cron_templates.len()
                ),

            SchedulerForm::Cloud =>
                list_utils::move_down(
                    &mut self.state.cloud_list_state,
                    self.state.clouds_protocols.len()
                ),

            SchedulerForm::Protocol => {
                let len = self.state.protocols().len();
                list_utils::move_down(&mut self.state.protocol_list_state, len);
            }
            _ => {}
        }
    }

    pub fn next(app: &mut App, mode: SchedulerForm) {
        let state = &mut app.components.scheduler_form.state;
        state.init_index_list(&mode);

        if app.state.mode == AppMode::SchedulerForm(SchedulerForm::Protocol) {
            state.protocol_list_state.select(None);
        }

        app.change_mode(AppMode::SchedulerForm(mode), app.state.mode.clone());
    }

    pub fn get_helper_text(&self, mode: &SchedulerForm) -> &'static str {
        match mode {
            _ => HELP,
        }
    }
}
