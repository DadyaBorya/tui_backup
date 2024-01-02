use tui::{ backend::Backend, Frame, layout::{ Layout, Constraint } };

use crate::{
    application::{ app::App, app_mode::{ AppMode, TemplateForm } },
    generator::{ popup, input_generator },
};

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let area = popup::popup(50, 35, "Save template".to_string(), f);

    let chunks = Layout::default()
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    let form = &mut app.components.template_form;

    let name_input = input_generator::input(
        "Name*".to_string(),
        form.state.name.to_owned(),
        "template 1".to_string(),
        app.state.mode == AppMode::TemplateForm(TemplateForm::Name)
    );
    f.render_widget(name_input, chunks[0]);

    let submit_button = input_generator::button(
        "Submit".to_string(),
        app.state.mode == AppMode::TemplateForm(TemplateForm::Submit)
    );
    f.render_widget(submit_button, chunks[1]);
}
