#[derive(Default)]
pub struct FilePriorityFormState {
    pub regex: String,
    pub content: String,
    pub priority: String,
}

impl FilePriorityFormState {
    pub fn init() -> Self {
        FilePriorityFormState::default()
    }
}
