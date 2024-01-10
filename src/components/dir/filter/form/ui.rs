use tui::{ backend::Backend, Frame, layout::{ Layout, Constraint } };

use crate::{
    application::{ app::App, mode::{ AppMode, DirFilterForm } },
    generator::{ popup, input_generator },
};

use super::component::DirFilterFormComponent;

impl DirFilterFormComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let area = popup::popup(50, 42, "Dir Filter".to_string(), f);

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

        let form = &mut app.components.dir_filter_form;

        let regex_input = input_generator::input(
            "Regex*".to_string(),
            form.state.regex.to_owned(),
            r"Sample File_(.*)\.xlsx".to_string(),
            app.state.mode == AppMode::DirFilterForm(DirFilterForm::Regex)
        );
        f.render_widget(regex_input, chunks[0]);

        let deep_input = input_generator::input(
            "Deep*".to_string(),
            form.state.deep.to_owned(),
            "2".to_string(),
            app.state.mode == AppMode::DirFilterForm(DirFilterForm::Deep)
        );
        f.render_widget(deep_input, chunks[1]);

        let submit_button = input_generator::button(
            "Submit".to_string(),
            app.state.mode == AppMode::DirFilterForm(DirFilterForm::Submit)
        );
        f.render_widget(submit_button, chunks[2]);
    }
}