use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::AppMode };

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let dir_priority = &mut app.components.dir_priority;
    match key_code {
        KeyCode::Esc => {
            dir_priority.state.list_state.select(None);
            app.change_mode(AppMode::FileList, AppMode::DirFilePriority);
        }
        KeyCode::Char('[') => {
            dir_priority.state.list_state.select(None);
            app.change_mode(AppMode::DirFilePriority, AppMode::DirFilePriority);
        }
        _ => {}
    }
    Ok(())
}
