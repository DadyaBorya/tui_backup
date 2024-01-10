use tui::{backend::Backend, layout::Rect, Frame};

use crate::{
    application::{app::App, mode::AppMode},
    generator::list_generator,
};

use super::component::DirFilterComponent;

impl DirFilterComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let dir_filter = &mut app.components.dir_filter;

        let list = list_generator::list(
            "Dir Filter".to_string(),
            AppMode::DirFilter == app.state.mode,
            dir_filter.state.rows(),
        );

        f.render_stateful_widget(list, chunks[1], &mut dir_filter.state.list_state);
    }
}
