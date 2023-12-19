use tui::{ backend::Backend, Frame, layout::{ Layout, Constraint } };

use crate::{
    application::{ app::App, app_mode::{ AppMode, SchedulerForm } },
    generator::{ popup, input_generator },
};

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    match &app.state.mode {
        AppMode::SchedulerForm(form) => {
            match form {
                SchedulerForm::Name | SchedulerForm::Speed | SchedulerForm::NextCron =>
                    first_part(app, f),
                _ => {}
            }
        }
        _ => {}
    }
}

fn first_part<B: Backend>(app: &mut App, f: &mut Frame<B>) {
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
