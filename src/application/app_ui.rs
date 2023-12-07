use tui::{ backend::Backend, Frame, layout::{ Layout, Direction, Constraint, Rect } };

use crate::{tab_component::tab_ui, helper_component::helper_ui};

use super::app::App;

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let main_chunks = get_main_chunks(f.size());

    tab_ui::ui(app, f, &main_chunks);

    helper_ui::ui(app, f, &main_chunks);
}

pub fn get_main_chunks(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(area)
}
