use tui::{ backend::Backend, Frame, layout::Rect };

use crate::{ application::{ app::App, app_mode::AppMode }, generator::list_generator };

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    let file_filter = &mut app.components.file_filter;

    let list = list_generator::list(
        "File Filter".to_string(),
        AppMode::FileFilter == app.state.mode,
        file_filter.state.rows()
    );

    f.render_stateful_widget(list, chunks[0], &mut file_filter.state.list_state);
}
