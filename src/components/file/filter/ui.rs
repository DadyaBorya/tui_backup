use tui::{ backend::Backend, Frame, layout::Rect };

use crate::{ application::{ app::App, mode::AppMode }, generator::list_generator };

use super::component::FileFilterComponent;

impl FileFilterComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>, chunk_index: usize) {
        let file_filter = &mut app.components.file_filter;
    
        let list = list_generator::list(
            "File Filter".to_string(),
            AppMode::FileFilter == app.state.mode,
            file_filter.state.rows()
        );
    
        f.render_stateful_widget(list, chunks[chunk_index], &mut file_filter.state.list_state);
    }
}
