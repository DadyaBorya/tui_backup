use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::AppMode };

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let dir_file_priority = &mut app.components.dir_file_priority;
    match key_code {
        KeyCode::Esc => {
            dir_file_priority.state.list_state.select(None);
            app.change_mode(AppMode::FileList, AppMode::DirFilePriority);
        }
        KeyCode::Char(']') => {
            dir_file_priority.state.list_state.select(None);
            app.change_mode(AppMode::DirPriority, AppMode::DirFilePriority);
        }
        _ => {}
    }
    Ok(())
}
