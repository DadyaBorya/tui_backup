use crossterm::event::KeyCode;

use crate::application::app::App;

use super::component::SchedulerListComponent;


impl SchedulerListComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        let template_list = &mut app.components.scheduler_list;
        match key_code {
            KeyCode::Esc => SchedulerListComponent::exit(app),
            KeyCode::Up => template_list.move_up(),
            KeyCode::Down => template_list.move_down(),
            KeyCode::Char('d') => template_list.delete(),
            KeyCode::Char('e') => SchedulerListComponent::edit(app),
            KeyCode::Char('i') => SchedulerListComponent::execute(app),
            _ => {}
        }
        Ok(())
    }
}
