use crossterm::event::KeyCode;

use crate::application::app::App;

use super::dir_file_priority_component::DirFilePriorityComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    match key_code {
        KeyCode::Esc => DirFilePriorityComponent::exit(app),
        KeyCode::Char(']') => DirFilePriorityComponent::next_component(app),
        _ => {}
    }
    Ok(())
}
