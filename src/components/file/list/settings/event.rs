use crossterm::event::KeyCode;

use crate::application::app::App;

use super::component::FileListSettingComponent;

impl FileListSettingComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        let settings = &mut app.components.file_list_settings;
        match key_code {
            KeyCode::Esc => FileListSettingComponent::exit(app),
            KeyCode::Up => settings.move_up(),
            KeyCode::Down => settings.move_down(),
            KeyCode::Char(' ') => settings.select(),
            _ => {}
        }
        Ok(())
    }
}
