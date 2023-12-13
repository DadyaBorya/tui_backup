use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::{ AppMode, DirFilePriorityForm } };

use super::dir_file_priority_form_component::DirFilePriorityFormComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let file_form = &mut app.components.dir_file_priority_form;
    match &mut app.state.mode {
        AppMode::DirFilePriorityForm(form) =>
            match form {
                DirFilePriorityForm::Regex => {
                    match key_code {
                        KeyCode::Esc =>
                            DirFilePriorityFormComponent::exit(app, DirFilePriorityForm::Regex),
                        KeyCode::Tab =>
                            DirFilePriorityFormComponent::next(
                                app,
                                DirFilePriorityForm::Deep,
                                DirFilePriorityForm::Regex
                            ),
                        KeyCode::Backspace => {
                            file_form.state.regex.pop();
                        }
                        KeyCode::Char(c) => file_form.state.regex.push(c),
                        _ => {}
                    }
                }
                DirFilePriorityForm::Deep => {
                    match key_code {
                        KeyCode::Esc =>
                            DirFilePriorityFormComponent::exit(app, DirFilePriorityForm::Deep),
                        KeyCode::Tab =>
                            DirFilePriorityFormComponent::next(
                                app,
                                DirFilePriorityForm::Priority,
                                DirFilePriorityForm::Deep
                            ),
                        KeyCode::BackTab =>
                            DirFilePriorityFormComponent::next(
                                app,
                                DirFilePriorityForm::Regex,
                                DirFilePriorityForm::Deep
                            ),
                        KeyCode::Backspace => {
                            file_form.state.deep.pop();
                        }
                        KeyCode::Char(c) => file_form.state.deep.push(c),
                        _ => {}
                    }
                }
                DirFilePriorityForm::Priority => {
                    match key_code {
                        KeyCode::Esc =>
                            DirFilePriorityFormComponent::exit(app, DirFilePriorityForm::Content),
                        KeyCode::Tab =>
                            DirFilePriorityFormComponent::next(
                                app,
                                DirFilePriorityForm::Content,
                                DirFilePriorityForm::Priority
                            ),
                        KeyCode::BackTab =>
                            DirFilePriorityFormComponent::next(
                                app,
                                DirFilePriorityForm::Deep,
                                DirFilePriorityForm::Priority
                            ),
                        KeyCode::Backspace => {
                            file_form.state.priority.pop();
                        }
                        KeyCode::Char(c) => file_form.state.priority.push(c),
                        _ => {}
                    }
                }
                DirFilePriorityForm::Content => {
                    match key_code {
                        KeyCode::Esc =>
                            DirFilePriorityFormComponent::exit(app, DirFilePriorityForm::Content),
                        KeyCode::Tab =>
                            DirFilePriorityFormComponent::next(
                                app,
                                DirFilePriorityForm::Submit,
                                DirFilePriorityForm::Content
                            ),
                        KeyCode::BackTab =>
                            DirFilePriorityFormComponent::next(
                                app,
                                DirFilePriorityForm::Priority,
                                DirFilePriorityForm::Content
                            ),
                        KeyCode::Backspace => {
                            file_form.state.content.pop();
                        }
                        KeyCode::Char(c) => file_form.state.content.push(c),
                        KeyCode::Enter => file_form.state.content.push('\n'),
                        _ => {}
                    }
                }
                DirFilePriorityForm::Submit => {
                    match key_code {
                        KeyCode::Esc =>
                            DirFilePriorityFormComponent::exit(app, DirFilePriorityForm::Submit),
                        KeyCode::BackTab =>
                            DirFilePriorityFormComponent::next(
                                app,
                                DirFilePriorityForm::Content,
                                DirFilePriorityForm::Submit
                            ),
                        KeyCode::Enter => DirFilePriorityFormComponent::add(app),
                        _ => {}
                    }
                }
            }
        _ => {}
    }
    Ok(())
}
