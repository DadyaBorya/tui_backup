use tui::{ backend::Backend, Frame, layout::{ Layout, Constraint } };

use crate::{
    application::{ app::App, app_mode::{ AppMode, FileFilterForm } },
    generator::{ popup, input_generator },
};

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let area = popup::popup(60, 58, f);

    let chunks = Layout::default()
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(6),
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref()
        )
        .split(area);

    let form = &mut app.components.file_filter_form;

    let regex_input = input_generator::input(
        "Regex*".to_string(),
        form.state.regex.to_owned(),
        r"Sample File_(.*)\.xlsx".to_string(),
        app.state.mode == AppMode::FileFilterForm(FileFilterForm::Regex)
    );
    f.render_widget(regex_input, chunks[0]);

    let deep_input = input_generator::input(
        "Deep*".to_string(),
        form.state.deep.to_owned(),
        "2".to_string(),
        app.state.mode == AppMode::FileFilterForm(FileFilterForm::Deep)
    );
    f.render_widget(deep_input, chunks[1]);

    let content_input = input_generator::input(
        "Content*".to_string(),
        form.state.content.to_owned(),
        "Paragraph*".to_string(),
        app.state.mode == AppMode::FileFilterForm(FileFilterForm::Content)
    );
    f.render_widget(content_input, chunks[2]);

    let submit_button = input_generator::button(
        "Submit".to_string(),
        app.state.mode == AppMode::FileFilterForm(FileFilterForm::Submit)
    );
    f.render_widget(submit_button, chunks[3]);
}
