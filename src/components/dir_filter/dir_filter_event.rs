use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::AppMode };

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let dir_filter = &mut app.components.dir_filter;
    match key_code {
        KeyCode::Esc => {
            dir_filter.state.list_state.select(None);
            app.change_mode(AppMode::FileList, AppMode::DirFilter);
        }
        KeyCode::Char('[') => {
            dir_filter.state.list_state.select(None);
            app.change_mode(AppMode::FileFilter, AppMode::DirFilter);
        }
        _ => {}
    }
    Ok(())
}
