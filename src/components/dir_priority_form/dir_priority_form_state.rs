#[derive(Default)]
pub struct DirPriorityFormState {
    pub regex: String,
    pub priority: String,
    pub deep: String,
}

impl DirPriorityFormState {
    pub fn init() -> Self {
        DirPriorityFormState::default()
    }
}
