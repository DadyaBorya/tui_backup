use tui::{ backend::Backend, Frame, layout::{ Rect, Layout, Direction, Constraint } };

use crate::{
    application::{ app::App, app_mode::AppMode },
    generator::table_generator,
    components::{
        file_filter::file_filter_ui,
        dir_filter::dir_filter_ui,
        dir_file_priority::dir_file_priority_ui,
        dir_priority::dir_priority_ui,
        file_priority::file_priority_ui,
    },
};

const HEADERS: [&'static str; 3] = ["", "Name", "Extension"];

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    let file_list = &mut app.components.file_list;
    let chunks = get_main_chunks(file_list.state.is_selected(), chunks);

    let table = table_generator
        ::table(
            HEADERS.to_vec(),
            &file_list.state.table_rows,
            file_list.state.current_path.to_str().unwrap(),
            app.state.mode == AppMode::FileList
        )
        .widths(&[Constraint::Length(3), Constraint::Length(40), Constraint::Min(10)]);

    f.render_stateful_widget(table, chunks[0], &mut file_list.state.table_state);

    if file_list.state.is_selected() {
        file_list_actions(&chunks, app, f);
    }
}

fn get_main_chunks(is_selected: bool, chunks: &Vec<Rect>) -> Vec<Rect> {
    match !is_selected {
        true =>
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(chunks[1]),
        false =>
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
                .split(chunks[1]),
    }
}

fn file_list_actions<B: Backend>(chunks: &Vec<Rect>, app: &mut App, f: &mut Frame<B>) {
    let action_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let entry = app.components.file_list.state.get_selected_entry().unwrap();
    app.components.file_filter.state.rules = entry.entry_file_filter.clone().unwrap_or_default();
    app.components.dir_filter.state.rules = entry.entry_dir_filter.clone().unwrap_or_default();
    app.components.dir_file_priority.state.rules = entry.entry_dir_file_priority
        .clone()
        .unwrap_or_default();
    app.components.dir_priority.state.rules = entry.entry_dir_priority.clone().unwrap_or_default();

    match entry.is_dir() {
        true =>
            match app.components.file_list.state.is_priority_mode {
                true => {
                    dir_file_priority_ui::ui(app, f, &action_chunks);
                    dir_priority_ui::ui(app, f, &action_chunks);
                }
                false => {
                    file_filter_ui::ui(app, f, &action_chunks);
                    dir_filter_ui::ui(app, f, &action_chunks);
                }
            }
        false => {
            file_priority_ui::ui(app, f, &action_chunks);
        }
    }
}
