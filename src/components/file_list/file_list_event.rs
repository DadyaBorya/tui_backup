use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::AppMode };

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let file_list = &mut app.components.file_list;
    match key_code {
        KeyCode::Esc => {
            file_list.state.table_state.select(None);
            app.change_mode(AppMode::Tab, AppMode::FileList);
        }
        KeyCode::Down => file_list.move_down(),
        KeyCode::Up => file_list.move_up(),
        KeyCode::Right => file_list.open()?,
        KeyCode::Left => file_list.close(),
        KeyCode::Char(' ') => file_list.select()?,
        KeyCode::Char('s') => file_list.select_deep()?,
        KeyCode::Char('a') => file_list.select_all(),
        KeyCode::Char('f') => {
            if file_list.state.is_selected_dir() {
                file_list.state.is_priority_mode = false;
                app.change_mode(AppMode::FileFilter, AppMode::FileList);
            }
        }
        KeyCode::Char('p') => {
            if file_list.state.is_selected() {
                file_list.state.is_priority_mode = true;

                match file_list.state.is_selected_dir() {
                    true => app.change_mode(AppMode::DirFilePriority, AppMode::FileList),
                    false => app.change_mode(AppMode::FilePriority, AppMode::FileList),
                }
            }
        }
        _ => {}
    }
    Ok(())
}
