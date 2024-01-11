use tui::{ backend::Backend, Frame, layout::{ Layout, Constraint } };

use crate::{
    application::{ app::App, mode::{ AppMode, DirPriorityForm } },
    generator::{ popup, input_generator },
};

use super::component::DirPriorityFormComponent;

impl DirPriorityFormComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let area = popup::popup(50, 50, "Dir Priority".to_string(), f);
    
        let chunks = Layout::default()
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ].as_ref()
            )
            .split(area);
    
        let form = &mut app.components.dir_priority_form;
    
        let regex_input = input_generator::input(
            "Regex*".to_string(),
            form.state.regex.to_owned(),
            r"Sample File_(.*)\.xlsx".to_string(),
            app.state.mode == AppMode::DirPriorityForm(DirPriorityForm::Regex)
        );
        f.render_widget(regex_input, chunks[0]);
    
        let deep_input = input_generator::input(
            "Deep".to_string(),
            form.state.deep.to_owned(),
            "2".to_string(),
            app.state.mode == AppMode::DirPriorityForm(DirPriorityForm::Deep)
        );
        f.render_widget(deep_input, chunks[1]);
    
        let priority_input = input_generator::input(
            "Priority*".to_string(),
            form.state.priority.to_owned(),
            "2".to_string(),
            app.state.mode == AppMode::DirPriorityForm(DirPriorityForm::Priority)
        );
        f.render_widget(priority_input, chunks[2]);
    
        let submit_button = input_generator::button(
            "Submit".to_string(),
            app.state.mode == AppMode::DirPriorityForm(DirPriorityForm::Submit)
        );
        f.render_widget(submit_button, chunks[3]);
    }
}
