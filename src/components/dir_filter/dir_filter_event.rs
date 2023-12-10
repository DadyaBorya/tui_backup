use crossterm::event::KeyCode;

use crate::application::app::App;

use super::dir_filter_component::DirFilterComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    match key_code {
        KeyCode::Esc => DirFilterComponent::exit(app),
        KeyCode::Char('[') => DirFilterComponent::prev_component(app),
        _ => {}
    }
    Ok(())
}
