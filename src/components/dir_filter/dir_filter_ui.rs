use tui::{ backend::Backend, Frame, layout::Rect };

use crate::{ application::{ app::App, app_mode::AppMode }, generator::list_generator };

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    let dir_filter = &mut app.components.dir_filter;

    let list = list_generator::list(
        "Dir Filter".to_string(),
        AppMode::DirFilter == app.state.mode,
        dir_filter.state.rows()
    );

    f.render_stateful_widget(list, chunks[1], &mut dir_filter.state.list_state);
}
