use crossterm::event::KeyCode;

use crate::application::app::App;

use super::scheduler_list_component::SchedulerListComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let template_list = &mut app.components.scheduler_list;
    match key_code {
        KeyCode::Esc => SchedulerListComponent::exit(app),
        KeyCode::Up => template_list.move_up(),
        KeyCode::Down => template_list.move_down(),
        _ => {}
    }
    Ok(())
}
