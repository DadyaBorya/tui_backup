use tui::{ backend::Backend, Frame, layout::{ Layout, Constraint } };

use crate::{
    application::{ app::App, mode::{ AppMode, FilePriorityForm } },
    generator::{ popup, input_generator },
};

use super::component::FilePriorityFormComponent;

impl FilePriorityFormComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let area = popup::popup(50, 50, "File Priority".to_string(), f);
    
        let chunks = Layout::default()
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(6),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ].as_ref()
            )
            .split(area);
    
        let form = &mut app.components.file_priority_form;
    
        let priority_input = input_generator::input(
            "Priority*".to_string(),
            form.state.priority.to_owned(),
            "2".to_string(),
            app.state.mode == AppMode::FilePriorityForm(FilePriorityForm::Priority)
        );
        f.render_widget(priority_input, chunks[0]);
    
        let content_input = input_generator::input(
            "Content".to_string(),
            form.state.content.to_owned(),
            "Paragraph".to_string(),
            app.state.mode == AppMode::FilePriorityForm(FilePriorityForm::Content)
        );
        f.render_widget(content_input, chunks[1]);
    
        let submit_button = input_generator::button(
            "Submit".to_string(),
            app.state.mode == AppMode::FilePriorityForm(FilePriorityForm::Submit)
        );
        f.render_widget(submit_button, chunks[2]);
    }
}
