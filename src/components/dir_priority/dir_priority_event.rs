use crossterm::event::KeyCode;

use crate::application::app::App;

use super::dir_priority_component::DirPriorityComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    match key_code {
        KeyCode::Esc => DirPriorityComponent::exit(app),
        KeyCode::Char('[') => DirPriorityComponent::prev_component(app),
        KeyCode::Char('n') => DirPriorityComponent::new_rule(app),
        KeyCode::Up => DirPriorityComponent::move_up(&mut app.components.dir_priority),
        KeyCode::Down => DirPriorityComponent::move_down(&mut app.components.dir_priority),
        KeyCode::Char('d') => DirPriorityComponent::delete(app),
        KeyCode::Char('e') => DirPriorityComponent::edit(app),
        _ => {}
    }
    Ok(())
}
