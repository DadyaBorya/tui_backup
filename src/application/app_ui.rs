use tui::{ backend::Backend, Frame, layout::{ Layout, Direction, Constraint, Rect } };

use crate::components::{
    helper::helper_ui,
    file_list::file_list_ui,
    tab::tab_ui,
    message_popup::message_popup_ui,
    file_filter_form::file_filter_form_ui, dir_filter_form::dir_filter_form_ui,
};

use super::{ app::App, app_mode::AppMode };

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let chunks = get_main_chunks(f.size());

    tab_ui::ui(app, f, &chunks);

    preview(app, f, &chunks);

    handle_current_app_mode(app, f, &chunks);

    helper_ui::ui(app, f, &chunks);
}

fn get_main_chunks(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(area)
}

fn preview<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    let index: usize = app.components.tabs.state.index;

    match index {
        0 => file_list_ui::ui(app, f, &chunks),
        _ => {}
    }
}

fn handle_current_app_mode<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    match app.state.mode {
        AppMode::Tab => tab_ui::ui(app, f, &chunks),
        AppMode::MessagePopup => message_popup_ui::ui(f, app),
        AppMode::FileFilterForm(_) => file_filter_form_ui::ui(app, f),
        AppMode::DirFilterForm(_) => dir_filter_form_ui::ui(app, f),
        _ => {}
    }
}
