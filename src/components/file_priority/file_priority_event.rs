use crossterm::event::KeyCode;

use crate::application::app::App;

use super::file_priority_component::FilePriorityComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    match key_code {
        KeyCode::Esc => FilePriorityComponent::exit(app),
        _ => {}
    }
    Ok(())
}
