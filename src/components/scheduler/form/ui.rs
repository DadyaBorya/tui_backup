use tui::{ backend::Backend, Frame, layout::{ Layout, Constraint, Direction } };

use crate::{
    application::{ app::App, mode::{ AppMode, SchedulerForm } },
    generator::{ popup, input_generator, list_generator },
};

use super::component::SchedulerFormComponent;

impl SchedulerFormComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        match &app.state.mode {
            AppMode::SchedulerForm(form) => {
                match form {
                    SchedulerForm::Name | SchedulerForm::Speed | SchedulerForm::NextCron =>
                    SchedulerFormComponent::name_part(app, f),
                    SchedulerForm::Cron | SchedulerForm::NextCloud | SchedulerForm::PrevName =>
                    SchedulerFormComponent::cron_part(app, f),
                    | SchedulerForm::Cloud
                    | SchedulerForm::Protocol
                    | SchedulerForm::Submit
                    | SchedulerForm::PrevCron => SchedulerFormComponent::cloud_part(app, f),
                }
            }
            _ => {}
        }
    }
    
    fn name_part<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let area = popup::popup(50, 42, "Create Scheduler".to_string(), f);
    
        let chunks = Layout::default()
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ].as_ref()
            )
            .split(area);
    
        let form = &mut app.components.scheduler_form;
    
        let name_input = input_generator::input(
            "Name*".to_string(),
            form.state.name.to_owned(),
            r"Name".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::Name)
        );
        f.render_widget(name_input, chunks[0]);
    
        let speed_input = input_generator::input(
            "Speed Limit(Mb/s)*".to_string(),
            form.state.speed.to_owned(),
            "100".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::Speed)
        );
        f.render_widget(speed_input, chunks[1]);
    
        let submit_button = input_generator::button(
            "Next".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::NextCron)
        );
        f.render_widget(submit_button, chunks[2]);
    }
    
    fn cron_part<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let area = popup::popup(50, 75, "Create Scheduler".to_string(), f);
    
        let chunks = Layout::default()
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(area);
    
        let form = &mut app.components.scheduler_form;
    
        let cron_input = input_generator::input(
            "Cron*".to_string(),
            form.state.cron.to_owned(),
            r"* * * * *".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::Cron)
        );
        f.render_widget(cron_input, chunks[0]);
    
        let list = list_generator::list(
            "Cron templates".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::Cron),
            form.state.cron_templates.to_owned()
        );
    
        f.render_stateful_widget(list, chunks[1], &mut form.state.cron_list_state);
    
        let buttons_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[2]);
    
        let back_button = input_generator::button(
            "Back".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::PrevName)
        );
    
        f.render_widget(back_button, buttons_chunks[0]);
    
        let next_button = input_generator::button(
            "Next".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::NextCloud)
        );
    
        f.render_widget(next_button, buttons_chunks[1]);
    }
    
    fn cloud_part<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let state = &mut app.components.scheduler_form.state;
    
        let area = popup::popup(50, 75, "Create Scheduler".to_string(), f);
    
        let chunks = Layout::default()
            .margin(2)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(area);
    
        let list_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
    
        let cloud_list = list_generator::list(
            "Clouds".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::Cloud),
            state.clouds()
        );
    
        f.render_stateful_widget(cloud_list, list_chunks[0], &mut state.cloud_list_state);
    
        let protocol_list = list_generator::list(
            "Protocols".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::Protocol),
            state.protocols()
        );
    
        f.render_stateful_widget(protocol_list, list_chunks[1], &mut state.protocol_list_state);
    
        let buttons_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);
    
        let back_button = input_generator::button(
            "Back".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::PrevCron)
        );
    
        f.render_widget(back_button, buttons_chunks[0]);
    
        let next_button = input_generator::button(
            "Submit".to_string(),
            app.state.mode == AppMode::SchedulerForm(SchedulerForm::Submit)
        );
    
        f.render_widget(next_button, buttons_chunks[1]);
    }
    
}