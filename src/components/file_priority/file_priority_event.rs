use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::AppMode };

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let file_priority = &mut app.components.file_priority;
    match key_code {
        KeyCode::Esc => {
            file_priority.state.list_state.select(None);
            app.change_mode(AppMode::FileList, AppMode::FilePriority);
        }
        _ => {}
    }
    Ok(())
}
