use crossterm::event::KeyCode;

use crate::application::app::App;

use super::component::DirPriorityComponent;

impl DirPriorityComponent {

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        let priority = &mut app.components.dir_priority;

        match key_code {
            KeyCode::Esc => DirPriorityComponent::exit(app),
            KeyCode::Char('[') => DirPriorityComponent::prev_component(app),
            KeyCode::Char('n') => DirPriorityComponent::new_rule(app),
            KeyCode::Up => priority.move_up(),
            KeyCode::Down => priority.move_down(),
            KeyCode::Char('d') => DirPriorityComponent::delete(app),
            KeyCode::Char('e') => DirPriorityComponent::edit(app),
            _ => {}
        }
        Ok(())
    }
}
