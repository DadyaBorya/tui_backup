use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::{ FileFilterForm, AppMode } };

use super::file_filter_form_component::FileFilterFormComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let file_form = &mut app.components.file_filter_form;
    match &mut app.state.mode {
        AppMode::FileFilterForm(form) =>
            match form {
                FileFilterForm::Regex => {
                    match key_code {
                        KeyCode::Esc => FileFilterFormComponent::exit(app, FileFilterForm::Regex),
                        KeyCode::Tab =>
                            FileFilterFormComponent::next(
                                app,
                                FileFilterForm::Deep,
                                FileFilterForm::Regex
                            ),
                        KeyCode::Backspace => {
                            file_form.state.regex.pop();
                        }
                        KeyCode::Char(c) => file_form.state.regex.push(c),
                        _ => {}
                    }
                }
                FileFilterForm::Deep => {
                    match key_code {
                        KeyCode::Esc => FileFilterFormComponent::exit(app, FileFilterForm::Deep),
                        KeyCode::Tab =>
                            FileFilterFormComponent::next(
                                app,
                                FileFilterForm::Content,
                                FileFilterForm::Deep
                            ),
                        KeyCode::BackTab =>
                            FileFilterFormComponent::next(
                                app,
                                FileFilterForm::Regex,
                                FileFilterForm::Deep
                            ),
                        KeyCode::Backspace => {
                            file_form.state.deep.pop();
                        }
                        KeyCode::Char(c) => file_form.state.deep.push(c),
                        _ => {}
                    }
                }
                FileFilterForm::Content => {
                    match key_code {
                        KeyCode::Esc => FileFilterFormComponent::exit(app, FileFilterForm::Content),
                        KeyCode::Tab =>
                            FileFilterFormComponent::next(
                                app,
                                FileFilterForm::Submit,
                                FileFilterForm::Content
                            ),
                        KeyCode::BackTab =>
                            FileFilterFormComponent::next(
                                app,
                                FileFilterForm::Deep,
                                FileFilterForm::Content
                            ),
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
                        KeyCode::Esc => FileFilterFormComponent::exit(app, FileFilterForm::Submit),
                        KeyCode::BackTab =>
                            FileFilterFormComponent::next(
                                app,
                                FileFilterForm::Content,
                                FileFilterForm::Submit
                            ),
                        KeyCode::Enter => FileFilterFormComponent::create(app),
                        _ => {}
                    }
                }
            }
        _ => {}
    }
    Ok(())
}
