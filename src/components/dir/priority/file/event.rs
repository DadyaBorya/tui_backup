use crossterm::event::KeyCode;

use crate::application::app::App;

use super::component::DirFilePriorityComponent;
impl DirFilePriorityComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        let priority = &mut app.components.dir_file_priority;
        match key_code {
            KeyCode::Esc => DirFilePriorityComponent::exit(app),
            KeyCode::Char(']') => DirFilePriorityComponent::next_component(app),
            KeyCode::Char('n') => DirFilePriorityComponent::new_rule(app),
            KeyCode::Down => priority.move_down(),
            KeyCode::Up => priority.move_up(),
            KeyCode::Char('d') => DirFilePriorityComponent::delete(app),
            KeyCode::Char('e') => DirFilePriorityComponent::edit(app),
            _ => {}
        }
        Ok(())
    }
}