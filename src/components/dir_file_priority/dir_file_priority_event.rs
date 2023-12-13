use crossterm::event::KeyCode;

use crate::application::app::App;

use super::dir_file_priority_component::DirFilePriorityComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let filter = &mut app.components.dir_file_priority;
    match key_code {
        KeyCode::Esc => DirFilePriorityComponent::exit(app),
        KeyCode::Char(']') => DirFilePriorityComponent::next_component(app),
        KeyCode::Char('n') => DirFilePriorityComponent::new_rule(app),
        KeyCode::Down => filter.move_down(),
        KeyCode::Up => filter.move_up(),
        KeyCode::Char('d') => DirFilePriorityComponent::delete(app),
        KeyCode::Char('e') => DirFilePriorityComponent::edit(app),
        _ => {}
    }
    Ok(())
}
