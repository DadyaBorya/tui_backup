use crossterm::event::KeyCode;

use crate::application::app::App;

use super::file_priority_component::FilePriorityComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let priority = &mut app.components.file_priority;
    match key_code {
        KeyCode::Esc => FilePriorityComponent::exit(app),
        KeyCode::Char('n') => FilePriorityComponent::new_rule(app),
        KeyCode::Down => priority.move_down(),
        KeyCode::Up => priority.move_up(),
        KeyCode::Char('d') => FilePriorityComponent::delete(app),
        KeyCode::Char('e') => FilePriorityComponent::edit(app),
        _ => {}
    }
    Ok(())
}
