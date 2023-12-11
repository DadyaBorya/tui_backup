#[derive(Default)]
pub struct DirFilePriorityFormState {
    pub regex: String,
    pub deep: String,
    pub priority: String,
    pub content: String,
}

impl DirFilePriorityFormState {
    pub fn init() -> Self {
        DirFilePriorityFormState::default()
    }
}
