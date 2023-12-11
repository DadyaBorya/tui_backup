#[derive(Default)]
pub struct DirFilterFormState {
    pub regex: String,
    pub deep: String
}

impl DirFilterFormState {
    pub fn init() -> Self {
        DirFilterFormState::default()
    }
}
        