#[derive(Default)]
pub struct FileFilterFormState {
    pub regex: String,
    pub deep: String,
    pub content: String
}

impl FileFilterFormState {
    pub fn init() -> Self {
        FileFilterFormState::default()
    }
}
        