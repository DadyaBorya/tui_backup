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
                        KeyCode::Enter => SchedulerFormComponent::next(app, SchedulerForm::Cron),
                        _ => {}
                    }
                }
                SchedulerForm::Cron => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::Tab => SchedulerFormComponent::next(app, SchedulerForm::PrevName),
                        KeyCode::Up => scheduler_form.move_up(SchedulerForm::Cron),
                        KeyCode::Down => scheduler_form.move_down(SchedulerForm::Cron),
                        KeyCode::Char(c) => scheduler_form.state.cron.push(c),
                        KeyCode::Backspace => {
                            scheduler_form.state.cron.pop();
                        }
                        KeyCode::Enter => scheduler_form.paste_current_cron(),
                        _ => {}
                    }
                }
                SchedulerForm::PrevName => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::BackTab => SchedulerFormComponent::next(app, SchedulerForm::Cron),
                        KeyCode::Tab => SchedulerFormComponent::next(app, SchedulerForm::NextCloud),
                        KeyCode::Enter => SchedulerFormComponent::next(app, SchedulerForm::Name),
                        _ => {}
                    }
                }
                SchedulerForm::NextCloud => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::BackTab =>
                            SchedulerFormComponent::next(app, SchedulerForm::PrevName),
                        KeyCode::Enter => SchedulerFormComponent::next(app, SchedulerForm::Cloud),
                        _ => {}
                    }
                }
                SchedulerForm::Cloud => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::Char(']') =>
                            SchedulerFormComponent::next(app, SchedulerForm::Protocol),
                        KeyCode::Tab => SchedulerFormComponent::next(app, SchedulerForm::PrevCron),
                        KeyCode::Up => scheduler_form.move_up(SchedulerForm::Cloud),
                        KeyCode::Down => scheduler_form.move_down(SchedulerForm::Cloud),
                        KeyCode::Char(' ') => scheduler_form.select_cloud(),
                        _ => {}
                    }
                }
                SchedulerForm::Protocol => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::Char('[') =>
                            SchedulerFormComponent::next(app, SchedulerForm::Cloud),
                        KeyCode::Tab => SchedulerFormComponent::next(app, SchedulerForm::PrevCron),
                        KeyCode::Up => scheduler_form.move_up(SchedulerForm::Protocol),
                        KeyCode::Down => scheduler_form.move_down(SchedulerForm::Protocol),
                        KeyCode::Char(' ') => scheduler_form.select_protocol(),
                        _ => {}
                    }
                }
                SchedulerForm::PrevCron => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::BackTab => SchedulerFormComponent::next(app, SchedulerForm::Cloud),
                        KeyCode::Tab => SchedulerFormComponent::next(app, SchedulerForm::Submit),
                        KeyCode::Enter => SchedulerFormComponent::next(app, SchedulerForm::Cron),
                        _ => {}
                    }
                }
                SchedulerForm::Submit => {
                    match key_code {
                        KeyCode::Esc => SchedulerFormComponent::exit(app),
                        KeyCode::BackTab =>
                            SchedulerFormComponent::next(app, SchedulerForm::PrevCron),
                            KeyCode::Enter => SchedulerFormComponent::add(app),
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
