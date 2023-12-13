use crossterm::event::KeyCode;

use crate::application::app::App;

use super::dir_filter_component::DirFilterComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let filter = &mut app.components.dir_filter;
    match key_code {
        KeyCode::Esc => DirFilterComponent::exit(app),
        KeyCode::Char('[') => DirFilterComponent::prev_component(app),
        KeyCode::Char('n') => DirFilterComponent::new_rule(app),
        KeyCode::Down => filter.move_down(),
        KeyCode::Up => filter.move_up(),
        KeyCode::Char('d') => DirFilterComponent::delete(app),
        KeyCode::Char('e') => DirFilterComponent::edit(app),
        _ => {}
    }
    Ok(())
}
