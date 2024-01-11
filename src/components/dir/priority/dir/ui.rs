use tui::{ backend::Backend, Frame, layout::Rect };

use crate::{ application::{ app::App, mode::AppMode }, generator::list_generator };

use super::component::DirPriorityComponent;

impl DirPriorityComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>, chunk_index: usize) {
        let dir_priority = &mut app.components.dir_priority;
    
        let list = list_generator::list(
            "Dir Priority".to_string(),
            AppMode::DirPriority == app.state.mode,
            dir_priority.state.rows()
        );
    
        f.render_stateful_widget(list, chunks[chunk_index], &mut dir_priority.state.list_state);
    }
    
}