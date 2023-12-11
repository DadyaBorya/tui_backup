use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::{ AppMode, FilePriorityForm } };

use super::file_priority_form_component::FilePriorityFormComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let file_form = &mut app.components.file_priority_form;
    match &mut app.state.mode {
        AppMode::FilePriorityForm(form) => {
            match form {
                FilePriorityForm::Regex => {
                    match key_code {
                        KeyCode::Esc =>
                            FilePriorityFormComponent::exit(app, FilePriorityForm::Regex),
                        KeyCode::Tab =>
                            FilePriorityFormComponent::next(
                                app,
                                FilePriorityForm::Priority,
                                FilePriorityForm::Regex
                            ),
                        KeyCode::Backspace => {
                            file_form.state.regex.pop();
                        }
                        KeyCode::Char(c) => file_form.state.regex.push(c),
                        _ => {}
                    }
                }
                FilePriorityForm::Priority => {
                    match key_code {
                        KeyCode::Esc =>
                            FilePriorityFormComponent::exit(app, FilePriorityForm::Regex),
                        KeyCode::Tab =>
                            FilePriorityFormComponent::next(
                                app,
                                FilePriorityForm::Content,
                                FilePriorityForm::Priority
                            ),
                        KeyCode::BackTab =>
                            FilePriorityFormComponent::next(
                                app,
                                FilePriorityForm::Regex,
                                FilePriorityForm::Priority
                            ),
                        KeyCode::Backspace => {
                            file_form.state.priority.pop();
                        }
                        KeyCode::Char(c) => file_form.state.priority.push(c),
                        _ => {}
                    }
                }
                FilePriorityForm::Content => {
                    match key_code {
                        KeyCode::Esc =>
                            FilePriorityFormComponent::exit(app, FilePriorityForm::Regex),
                        KeyCode::Tab =>
                            FilePriorityFormComponent::next(
                                app,
                                FilePriorityForm::Submit,
                                FilePriorityForm::Content
                            ),
                        KeyCode::BackTab =>
                            FilePriorityFormComponent::next(
                                app,
                                FilePriorityForm::Priority,
                                FilePriorityForm::Content
                            ),
                        KeyCode::Backspace => {
                            file_form.state.content.pop();
                        }
                        KeyCode::Enter => file_form.state.content.push('\n'),
                        KeyCode::Char(c) => file_form.state.content.push(c),
                        _ => {}
                    }
                }
                FilePriorityForm::Submit => {
                    match key_code {
                        KeyCode::Esc =>
                            FilePriorityFormComponent::exit(app, FilePriorityForm::Submit),
                        KeyCode::BackTab =>
                            FilePriorityFormComponent::next(
                                app,
                                FilePriorityForm::Content,
                                FilePriorityForm::Submit
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
