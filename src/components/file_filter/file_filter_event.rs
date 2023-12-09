use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::AppMode };

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let file_filter = &mut app.components.file_filter;
    match key_code {
        KeyCode::Esc => {
            file_filter.state.list_state.select(None);
            app.change_mode(AppMode::FileList, AppMode::FileFilter);
        }
        KeyCode::Tab => {
            file_filter.state.list_state.select(None);
            app.change_mode(AppMode::DirFilter, AppMode::FileList);
        }
        _ => {}
    }
    Ok(())
}
