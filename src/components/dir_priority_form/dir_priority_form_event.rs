use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::{ AppMode, DirPriorityForm } };

use super::dir_priority_form_component::DirPriorityFormComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let dir_form = &mut app.components.dir_priority_form;
    match &mut app.state.mode {
        AppMode::DirPriorityForm(form) => {
            match form {
                DirPriorityForm::Regex => {
                    match key_code {
                        KeyCode::Esc => DirPriorityFormComponent::exit(app, DirPriorityForm::Regex),
                        KeyCode::Tab =>
                            DirPriorityFormComponent::next(
                                app,
                                DirPriorityForm::Deep,
                                DirPriorityForm::Regex
                            ),
                        KeyCode::Backspace => {
                            dir_form.state.regex.pop();
                        }
                        KeyCode::Char(c) => dir_form.state.regex.push(c),
                        _ => {}
                    }
                }
                DirPriorityForm::Deep => {
                    match key_code {
                        KeyCode::Esc => DirPriorityFormComponent::exit(app, DirPriorityForm::Regex),
                        KeyCode::Tab =>
                            DirPriorityFormComponent::next(
                                app,
                                DirPriorityForm::Priority,
                                DirPriorityForm::Deep
                            ),
                        KeyCode::BackTab =>
                            DirPriorityFormComponent::next(
                                app,
                                DirPriorityForm::Regex,
                                DirPriorityForm::Deep
                            ),
                        KeyCode::Backspace => {
                            dir_form.state.deep.pop();
                        }
                        KeyCode::Char(c) => dir_form.state.deep.push(c),
                        _ => {}
                    }
                }
                DirPriorityForm::Priority => {
                    match key_code {
                        KeyCode::Esc => DirPriorityFormComponent::exit(app, DirPriorityForm::Regex),
                        KeyCode::Tab =>
                            DirPriorityFormComponent::next(
                                app,
                                DirPriorityForm::Submit,
                                DirPriorityForm::Priority
                            ),
                        KeyCode::BackTab =>
                            DirPriorityFormComponent::next(
                                app,
                                DirPriorityForm::Deep,
                                DirPriorityForm::Priority
                            ),
                        KeyCode::Backspace => {
                            dir_form.state.priority.pop();
                        }
                        KeyCode::Char(c) => dir_form.state.priority.push(c),
                        _ => {}
                    }
                }
                DirPriorityForm::Submit => {
                    match key_code {
                        KeyCode::Esc =>
                            DirPriorityFormComponent::exit(app, DirPriorityForm::Submit),
                        KeyCode::BackTab =>
                            DirPriorityFormComponent::next(
                                app,
                                DirPriorityForm::Priority,
                                DirPriorityForm::Submit
                            ),
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
