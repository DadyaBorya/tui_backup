use crossterm::event::KeyCode;

use crate::application::app::App;

use super::template_list_component::TemplateListComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let template_list = &mut app.components.template_list;
    match key_code {
        KeyCode::Esc => TemplateListComponent::exit(app),
        KeyCode::Down => template_list.move_down(),
        KeyCode::Up => template_list.move_up(),
        KeyCode::Char('d') => template_list.delete(),
        KeyCode::Char('e') => TemplateListComponent::edit(app),
        KeyCode::Char('c') => TemplateListComponent::create_scheduler(app),
        _ => {}
    }
    Ok(())
}
