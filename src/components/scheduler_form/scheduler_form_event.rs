use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::{ AppMode, SchedulerForm } };

use super::scheduler_form_component::SchedulerFormComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let scheduler_form = &mut app.components.scheduler_form;

    match &mut app.state.mode {
        AppMode::SchedulerForm(form) => {
            match form {
                SchedulerForm::Name => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::Char(c) => scheduler_form.state.name.push(c),
                        KeyCode::Backspace => {
                            scheduler_form.state.name.pop();
                        }
                        KeyCode::Tab => SchedulerFormComponent::next(app, SchedulerForm::Speed),
                        _ => {}
                    }
                }
                SchedulerForm::Speed => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::Char(c) => scheduler_form.state.speed.push(c),
                        KeyCode::Backspace => {
                            scheduler_form.state.speed.pop();
                        }
                        KeyCode::Tab => SchedulerFormComponent::next(app, SchedulerForm::NextCron),
                        KeyCode::BackTab => SchedulerFormComponent::next(app, SchedulerForm::Name),
                        _ => {}
                    }
                }
                SchedulerForm::NextCron => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::BackTab => SchedulerFormComponent::next(app, SchedulerForm::Speed),
                        _ => {}
                    }
                }
                SchedulerForm::Cron => todo!(),
                SchedulerForm::Cloud => todo!(),
                SchedulerForm::Protocol => todo!(),
                SchedulerForm::NextCloud => todo!(),
            }
        }
        _ => {}
    }
    Ok(())
}
