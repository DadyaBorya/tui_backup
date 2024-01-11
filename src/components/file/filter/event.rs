use crossterm::event::KeyCode;

use crate::application::app::App;

use super::component::FileFilterComponent;

impl FileFilterComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Esc => FileFilterComponent::exit(app),
            KeyCode::Enter => FileFilterComponent::select_list(app),
            KeyCode::Char('n') => FileFilterComponent::new_rule(app),
            KeyCode::Down => FileFilterComponent::move_down(app),
            KeyCode::Up => FileFilterComponent::move_up(app),
            KeyCode::Char('d') => FileFilterComponent::delete(app),
            KeyCode::Char('e') => FileFilterComponent::edit(app),
            _ => {}
        }
        Ok(())
    }
}
