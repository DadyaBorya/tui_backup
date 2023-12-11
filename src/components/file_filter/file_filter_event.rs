use crossterm::event::KeyCode;

use crate::application::app::App;

use super::file_filter_component::FileFilterComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    match key_code {
        KeyCode::Esc => FileFilterComponent::exit(app),
        KeyCode::Char(']') => FileFilterComponent::next_component(app),
        KeyCode::Char('n') => FileFilterComponent::new_rule(app),
        _ => {}
    }
    Ok(())
}
