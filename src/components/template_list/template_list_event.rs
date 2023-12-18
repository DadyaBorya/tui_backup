use crossterm::event::KeyCode;

use crate::application::app::App;

use super::template_list_component::TemplateListComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    match key_code {
        KeyCode::Esc => TemplateListComponent::exit(app),
        _ => {}
    }
    Ok(())
}