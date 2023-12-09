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

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
