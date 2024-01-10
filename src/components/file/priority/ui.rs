use tui::{ backend::Backend, Frame, layout::Rect };

use crate::{ application::{ app::App, mode::AppMode }, generator::list_generator };

use super::component::FilePriorityComponent;

impl FilePriorityComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let file_priority = &mut app.components.file_priority;
    
        let list = list_generator::list(
            "File Priority".to_string(),
            AppMode::FilePriority == app.state.mode,
            file_priority.state.rows()
        );
    
        f.render_stateful_widget(list, chunks[0], &mut file_priority.state.list_state);
    }
}


