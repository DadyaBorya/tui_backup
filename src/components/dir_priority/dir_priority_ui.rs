use tui::{ backend::Backend, Frame, layout::Rect };

use crate::{ application::{ app::App, app_mode::AppMode }, generator::list_generator };

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    let dir_priority = &mut app.components.dir_priority;

    let list = list_generator::list(
        "Dir Priority".to_string(),
        AppMode::DirPriority == app.state.mode,
        dir_priority.state.rows()
    );

    f.render_stateful_widget(list, chunks[1], &mut dir_priority.state.list_state);
}
