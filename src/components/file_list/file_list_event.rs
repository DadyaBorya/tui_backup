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
        _ => {}
    }
    Ok(())
}
