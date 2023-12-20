use crate::{
    application::{ app_mode::{ SchedulerForm, AppMode }, app::App },
    utils::list_utils,
    models::scheduler::Scheduler,
    components::message_popup::message_popup_components::MessagePopupComponent,
};

use super::scheduler_form_state::SchedulerFormState;

const HELP_NAME: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next |";
const HELP_SPEED: &'static str = "| ESC~Exit | a-z0-9~Input | TAB~Next | BACKTAB~Prev |";
const HELP_NEXT_CRON: &'static str = "| ESC~Exit | ENTER~Select | BACKTAB~Prev |";
const HELP_CRON: &'static str = "| ESC~Exit | ↑ ↓ Move | a-z0-9~Input | ENTER~Select | TAB~Next |";
const HELP_PREV_NAME: &'static str = "| ESC~Exit | ENTER~Back | TAB~Next | BACKTAB~Prev |";
const HELP_NEXT_CLOUD: &'static str = "| ESC~Exit | ENTER~Next | BACKTAB~Prev |";
const HELP_CLOUD: &'static str = "| ESC~Exit | SPACE~Select | TAB~Next | ]~Protocols |";
const HELP_PROTOCOL: &'static str = "| ESC~Exit | SPACE~Select | TAB~Next | [~Clouds |";
const HELP_PREV_CRON: &'static str = "| ESC~Exit | TAB~Next | BACKTAB~Prev | ENTER~Back |";
const HELP_SUBMIT: &'static str = "| ESC~Exit | ENTER~Submit | BACKTAB~Prev |";

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
            SchedulerForm::Name => HELP_NAME,
            SchedulerForm::Speed => HELP_SPEED,
            SchedulerForm::NextCron => HELP_NEXT_CRON,
            SchedulerForm::Cron => HELP_CRON,
            SchedulerForm::NextCloud => HELP_NEXT_CLOUD,
            SchedulerForm::PrevName => HELP_PREV_NAME,
            SchedulerForm::Cloud => HELP_CLOUD,
            SchedulerForm::Protocol => HELP_PROTOCOL,
            SchedulerForm::PrevCron => HELP_PREV_CRON,
            SchedulerForm::Submit => HELP_SUBMIT,
        }
    }
}
