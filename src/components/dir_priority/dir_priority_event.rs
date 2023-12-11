use crossterm::event::KeyCode;

use crate::application::app::App;

use super::dir_priority_component::DirPriorityComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    match key_code {
        KeyCode::Esc => DirPriorityComponent::exit(app),
        KeyCode::Char('[') => DirPriorityComponent::prev_component(app),
        KeyCode::Char('n') => DirPriorityComponent::new_rule(app),
        _ => {}
    }
    Ok(())
}
