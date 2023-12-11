use tui::{ backend::Backend, Frame, layout::{ Layout, Constraint } };

use crate::{
    application::{ app::App, app_mode::{ AppMode, FilePriorityForm } },
    generator::{ popup, input_generator },
};

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let area = popup::popup(50, 60, f);

    let chunks = Layout::default()
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(area);

    let form = &mut app.components.file_priority_form;
    let regex_input = input_generator::input(
        "Regex*".to_string(),
        form.state.regex.to_owned(),
        r"Sample File_(.*)\.xlsx".to_string(),
        app.state.mode == AppMode::FilePriorityForm(FilePriorityForm::Regex)
    );
    f.render_widget(regex_input, chunks[0]);

    let priority_input = input_generator::input(
        "Priority*".to_string(),
        form.state.priority.to_owned(),
        "2".to_string(),
        app.state.mode == AppMode::FilePriorityForm(FilePriorityForm::Priority)
    );
    f.render_widget(priority_input, chunks[1]);

    let content_input = input_generator::input(
        "Content*".to_string(),
        form.state.content.to_owned(),
        "Paragraph*".to_string(),
        app.state.mode == AppMode::FilePriorityForm(FilePriorityForm::Content)
    );
    f.render_widget(content_input, chunks[2]);

    let submit_button = input_generator::button(
        "Submit".to_string(),
        app.state.mode == AppMode::FilePriorityForm(FilePriorityForm::Submit)
    );
    f.render_widget(submit_button, chunks[3]);
}
