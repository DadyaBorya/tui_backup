use tui::{ backend::Backend, Frame, layout::Rect };

use crate::{ application::{ app::App, mode::AppMode }, generator::list_generator };

use super::component::DirFilePriorityComponent;


impl DirFilePriorityComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let dir_file_priority = &mut app.components.dir_file_priority;
    
        let list = list_generator::list(
            "File Priority".to_string(),
            AppMode::DirFilePriority == app.state.mode,
            dir_file_priority.state.rows()
        );
    
        f.render_stateful_widget(list, chunks[0], &mut dir_file_priority.state.list_state);
    }
}


