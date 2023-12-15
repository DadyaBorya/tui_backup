use crossterm::event::KeyCode;

use crate::application::{ app::App, app_mode::{ AppMode, CreateTemplateForm } };

use super::create_template_form_component::CreateTemplateFormComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let template_form = &mut app.components.create_template_form;
    match &mut app.state.mode {
        AppMode::CreateTemplateForm(form) => {
            match form {
                CreateTemplateForm::Name => {
                    match key_code {
                        KeyCode::Esc =>
                            CreateTemplateFormComponent::exit(app, CreateTemplateForm::Name),
                        KeyCode::Char(c) => template_form.state.name.push(c),
                        KeyCode::Backspace => {
                            template_form.state.name.pop();
                        }
                        KeyCode::Tab => {
                            CreateTemplateFormComponent::next(
                                app,
                                CreateTemplateForm::Submit,
                                CreateTemplateForm::Name
                            );
                        }
                        _ => {}
                    }
                }
                CreateTemplateForm::Submit => {
                    match key_code {
                        KeyCode::Esc =>
                            CreateTemplateFormComponent::exit(app, CreateTemplateForm::Submit),
                        KeyCode::BackTab => {
                            CreateTemplateFormComponent::next(
                                app,
                                CreateTemplateForm::Name,
                                CreateTemplateForm::Submit
                            );
                        }

                        KeyCode::Enter =>  CreateTemplateFormComponent::submit(app),
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
