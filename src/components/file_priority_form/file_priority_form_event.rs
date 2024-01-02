use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::{ AppMode, FilePriorityForm } };

use super::file_priority_form_component::FilePriorityFormComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let file_form = &mut app.components.file_priority_form;
    match &mut app.state.mode {
        AppMode::FilePriorityForm(form) => {
            match form {
                FilePriorityForm::Priority => {
                    match key_code {
                        KeyCode::Esc => FilePriorityFormComponent::exit(app),
                        KeyCode::Tab =>
                            FilePriorityFormComponent::next(app, FilePriorityForm::Content),
                        KeyCode::Backspace => {
                            file_form.state.priority.pop();
                        }
                        KeyCode::Char(c) => file_form.state.priority.push(c),
                        _ => {}
                    }
                }
                FilePriorityForm::Content => {
                    match key_code {
                        KeyCode::Esc => FilePriorityFormComponent::exit(app),
                        KeyCode::Tab =>
                            FilePriorityFormComponent::next(app, FilePriorityForm::Submit),
                        KeyCode::BackTab =>
                            FilePriorityFormComponent::next(app, FilePriorityForm::Priority),
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
                        KeyCode::Esc => FilePriorityFormComponent::exit(app),
                        KeyCode::BackTab =>
                            FilePriorityFormComponent::next(app, FilePriorityForm::Content),
                        KeyCode::Enter => FilePriorityFormComponent::add(app),
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
