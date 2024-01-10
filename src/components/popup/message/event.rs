use crossterm::event::KeyCode;

use crate::application::{ app::App, mode::AppMode };

use super::component::MessagePopupComponent;

impl MessagePopupComponent {
    pub fn event(app: &mut App, key_code: KeyCode) {
        match key_code {
            KeyCode::Esc => app.change_mode(app.state.prev_mode.clone(), AppMode::MessagePopup),
            _ => {}
        }
    }
    
}