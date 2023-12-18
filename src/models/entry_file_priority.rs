#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct EntryFilePriority {
    pub content: String,
    pub priority: usize,
    pub root: String,
}

impl EntryFilePriority {
    pub fn row(&self) -> String {
        format!("priority: {priority}\n{content}", content = self.content, priority = self.priority)
    }
}
