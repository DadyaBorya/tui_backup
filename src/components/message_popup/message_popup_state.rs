#[derive(Default)]
pub struct MessagePopupState {
    pub title: String,
    pub message: String,
    pub w: u16,
    pub h: u16,
}

impl MessagePopupState {
    pub fn init() -> Self {
        MessagePopupState::default()
    }

    pub fn edit(&mut self, title: String, message: String, w: u16, h: u16) {
        self.title = title;
        self.message = message;
        self.w = w;
        self.h = h;
    }
}
