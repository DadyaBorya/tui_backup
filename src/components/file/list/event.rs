use crossterm::event::KeyCode;

use crate::application::{ app::App, mode::AppMode };

use super::component::FileListComponent;

impl FileListComponent {
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
            KeyCode::Char('f') => FileListComponent::open_edit_settings(app),
            KeyCode::Char('p') => FileListComponent::open_settings(app),
            KeyCode::Char('c') => FileListComponent::save(app),
            KeyCode::Char('n') => file_list.clear()?,
            _ => {}
        }
        Ok(())
    }
}
