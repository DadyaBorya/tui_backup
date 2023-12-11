use crate::application::{ app::App, app_mode::AppMode };

use super::message_popup_state::MessagePopupState;

const HELP: &'static str = "| ESC~Back |";

pub struct MessagePopupComponent {
    pub state: MessagePopupState,
}

impl MessagePopupComponent {
    pub fn init() -> Self {
        MessagePopupComponent {
            state: MessagePopupState::init(),
        }
    }

    pub fn show_vec(app: &mut App, vec: Vec<String>, prev_mode: AppMode) {
        let h = 15 + 3 * vec.len();
        let text = vec.join("\n");
        app.components.message_popup.state.edit("Error".to_string(), text, 60, h as u16);
        app.change_mode(AppMode::MessagePopup, prev_mode);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
