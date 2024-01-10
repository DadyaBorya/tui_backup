use crossterm::event::KeyCode;

use crate::application::app::App;

use super::component::FileFilterComponent;


impl FileFilterComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        let filter = &mut app.components.file_filter;
        match key_code {
            KeyCode::Esc => FileFilterComponent::exit(app),
            KeyCode::Char(']') => FileFilterComponent::next_component(app),
            KeyCode::Char('n') => FileFilterComponent::new_rule(app),
            KeyCode::Down => filter.move_down(),
            KeyCode::Up => filter.move_up(),
            KeyCode::Char('d') => FileFilterComponent::delete(app),
            KeyCode::Char('e') => FileFilterComponent::edit(app),
            _ => {}
        }
        Ok(())
    }
}
