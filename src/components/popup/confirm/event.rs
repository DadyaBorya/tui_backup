use crossterm::event::KeyCode;

use crate::application::{
    app::App,
    mode::{AppMode, Confirm},
};

use super::component::ConfirmPopupComponent;

impl ConfirmPopupComponent {
    pub fn event(app: &mut App, key_code: KeyCode) {
        match &mut app.state.mode {
            AppMode::Confirm(mode) => match mode {
                Confirm::Cancel => match key_code {
                    KeyCode::Esc => ConfirmPopupComponent::exit(app),
                    KeyCode::Tab => ConfirmPopupComponent::next(app, Confirm::Submit),
                    KeyCode::Enter => ConfirmPopupComponent::exit(app),
                    _ => {}
                },
                Confirm::Submit => match key_code {
                    KeyCode::Esc => ConfirmPopupComponent::exit(app),
                    KeyCode::BackTab => ConfirmPopupComponent::next(app, Confirm::Cancel),
                    KeyCode::Enter => ConfirmPopupComponent::submit(app),
                    _ => {}
                },
            },
            _ => {}
        }
    }
}
