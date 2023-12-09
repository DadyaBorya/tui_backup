use tui::{ backend::Backend, Frame, layout::{ Rect, Layout, Direction, Constraint } };

use crate::{ application::{ app::App, app_mode::AppMode }, generator::table_generator };

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
}

fn get_main_chunks(is_selected: bool, chunks: &Vec<Rect>) -> Vec<Rect> {
    match !is_selected {
        true =>
            Layout::default()
                .margin(1)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(chunks[1]),
        false =>
            Layout::default()
                .margin(1)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
                .split(chunks[1]),
    }
}
