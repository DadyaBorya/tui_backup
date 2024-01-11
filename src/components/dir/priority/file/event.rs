use crossterm::event::KeyCode;

use crate::application::app::App;

use super::component::DirFilePriorityComponent;
impl DirFilePriorityComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Esc => DirFilePriorityComponent::exit(app),
            KeyCode::Enter => DirFilePriorityComponent::select_list(app),
            KeyCode::Char('n') => DirFilePriorityComponent::new_rule(app),
            KeyCode::Down => DirFilePriorityComponent::move_down(app),
            KeyCode::Up => DirFilePriorityComponent::move_up(app),
            KeyCode::Char('d') => DirFilePriorityComponent::delete(app),
            KeyCode::Char('e') => DirFilePriorityComponent::edit(app),
            _ => {}
        }
        Ok(())
    }
}