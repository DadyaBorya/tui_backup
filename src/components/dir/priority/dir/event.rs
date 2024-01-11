use crossterm::event::KeyCode;

use crate::application::app::App;

use super::component::DirPriorityComponent;

impl DirPriorityComponent {

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {

        match key_code {
            KeyCode::Esc => DirPriorityComponent::exit(app),
            KeyCode::Enter => DirPriorityComponent::select_list(app),
            KeyCode::Char('n') => DirPriorityComponent::new_rule(app),
            KeyCode::Up => DirPriorityComponent::move_up(app),
            KeyCode::Down => DirPriorityComponent::move_down(app),
            KeyCode::Char('d') => DirPriorityComponent::delete(app),
            KeyCode::Char('e') => DirPriorityComponent::edit(app),
            _ => {}
        }
        Ok(())
    }
}
