use crossterm::event::KeyCode;

use crate::application::{ app::App, mode::{ FileFilterForm, AppMode } };

use super::component::FileFilterFormComponent;

impl FileFilterFormComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        let file_form = &mut app.components.file_filter_form;
        match &mut app.state.mode {
            AppMode::FileFilterForm(form) =>
                match form {
                    FileFilterForm::Regex => {
                        match key_code {
                            KeyCode::Esc => FileFilterFormComponent::exit(app),
                            KeyCode::Tab => FileFilterFormComponent::next(app, FileFilterForm::Deep),
                            KeyCode::Backspace => {
                                file_form.state.regex.pop();
                            }
                            KeyCode::Char(c) => file_form.state.regex.push(c),
                            _ => {}
                        }
                    }
                    FileFilterForm::Deep => {
                        match key_code {
                            KeyCode::Esc => FileFilterFormComponent::exit(app),
                            KeyCode::Tab => FileFilterFormComponent::next(app, FileFilterForm::Content),
                            KeyCode::BackTab =>
                                FileFilterFormComponent::next(app, FileFilterForm::Regex),
                            KeyCode::Backspace => {
                                file_form.state.deep.pop();
                            }
                            KeyCode::Char(c) => file_form.state.deep.push(c),
                            _ => {}
                        }
                    }
                    FileFilterForm::Content => {
                        match key_code {
                            KeyCode::Esc => FileFilterFormComponent::exit(app),
                            KeyCode::Tab => FileFilterFormComponent::next(app, FileFilterForm::Submit),
                            KeyCode::BackTab =>
                                FileFilterFormComponent::next(app, FileFilterForm::Deep),
                            KeyCode::Backspace => {
                                file_form.state.content.pop();
                            }
                            KeyCode::Char(c) => file_form.state.content.push(c),
                            KeyCode::Enter => file_form.state.content.push('\n'),
                            _ => {}
                        }
                    }
                    FileFilterForm::Submit => {
                        match key_code {
                            KeyCode::Esc => FileFilterFormComponent::exit(app),
                            KeyCode::BackTab =>
                                FileFilterFormComponent::next(app, FileFilterForm::Content),
                            KeyCode::Enter => FileFilterFormComponent::add(app),
                            _ => {}
                        }
                    }
                }
            _ => {}
        }
        Ok(())
    }
}
