use crossterm::event::KeyCode;

use crate::application::{ app::App, mode::{ AppMode, TemplateForm } };

use super::component::TemplateFormComponent;

impl TemplateFormComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        let template_form = &mut app.components.template_form;
        match &mut app.state.mode {
            AppMode::TemplateForm(form) => {
                match form {
                    TemplateForm::Name => {
                        match key_code {
                            KeyCode::Esc => TemplateFormComponent::exit(app),
                            KeyCode::Char(c) => template_form.state.name.push(c),
                            KeyCode::Backspace => {
                                template_form.state.name.pop();
                            }
                            KeyCode::Tab => {
                                TemplateFormComponent::next(app, TemplateForm::Submit);
                            }
                            _ => {}
                        }
                    }
                    TemplateForm::Submit => {
                        match key_code {
                            KeyCode::Esc => TemplateFormComponent::exit(app),
                            KeyCode::BackTab => {
                                TemplateFormComponent::next(app, TemplateForm::Name);
                            }
    
                            KeyCode::Enter => TemplateFormComponent::submit(app),
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
