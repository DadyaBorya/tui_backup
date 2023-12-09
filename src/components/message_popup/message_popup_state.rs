#[derive(Default)]
pub struct MessagePopupState {
    pub title: String,
    pub message: String,
}

impl MessagePopupState {
    pub fn init() -> Self {
        MessagePopupState::default()
    }

    pub fn edit(&mut self, title: String, message: String) {
        self.title = title;
        self.message = message;
    }
}
