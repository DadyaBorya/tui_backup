use crossterm::event::KeyCode;

use crate::application::{
    app::App,
    mode::{AppMode, DirFilterForm},
};

use super::component::DirFilterFormComponent;

impl DirFilterFormComponent {
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        let dir_form = &mut app.components.dir_filter_form;
        match &mut app.state.mode {
            AppMode::DirFilterForm(form) => match form {
                DirFilterForm::Regex => match key_code {
                    KeyCode::Esc => DirFilterFormComponent::exit(app),
                    KeyCode::Tab => DirFilterFormComponent::next(app, DirFilterForm::Deep),
                    KeyCode::Backspace => {
                        dir_form.state.regex.pop();
                    }
                    KeyCode::Char(c) => dir_form.state.regex.push(c),
                    _ => {}
                },
                DirFilterForm::Deep => match key_code {
                    KeyCode::Esc => DirFilterFormComponent::exit(app),
                    KeyCode::Tab => DirFilterFormComponent::next(app, DirFilterForm::Submit),
                    KeyCode::BackTab => DirFilterFormComponent::next(app, DirFilterForm::Regex),
                    KeyCode::Backspace => {
                        dir_form.state.deep.pop();
                    }
                    KeyCode::Char(c) => dir_form.state.deep.push(c),
                    _ => {}
                },
                DirFilterForm::Submit => match key_code {
                    KeyCode::Esc => DirFilterFormComponent::exit(app),
                    KeyCode::BackTab => DirFilterFormComponent::next(app, DirFilterForm::Deep),
                    KeyCode::Enter => DirFilterFormComponent::add(app),
                    _ => {}
                },
            },
            _ => {}
        }
        Ok(())
    }
}
