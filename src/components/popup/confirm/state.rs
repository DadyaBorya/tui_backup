use crate::application::mode::AppMode;

pub struct ConfirmPopupState {
    pub title: String,
    pub message: String,
    pub prev_mode: AppMode,
    pub next_mode: AppMode,
}

impl Default for ConfirmPopupState {
    fn default() -> Self {
        ConfirmPopupState {
            title: String::default(),
            message: String::default(),
            prev_mode: AppMode::default(),
            next_mode: AppMode::default(),
        }
    }
}

impl ConfirmPopupState {
    pub fn init() -> Self {
        ConfirmPopupState::default()
    }

    pub fn edit(
        &mut self,
        title: String,
        message: String,
        prev_mode: AppMode,
        next_mode: AppMode,
    ) {
        self.title = title;
        self.message = message;
        self.next_mode = next_mode;
        self.prev_mode = prev_mode;
    }
}
