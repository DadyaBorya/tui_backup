#[derive(Default, Clone)]
pub struct EntryDirPriority {
    pub regex: String,
    pub deep: usize,
    pub priority: usize
}

impl EntryDirPriority {
    pub fn row(&self) -> String {
        format!(
            "regex: {regex} deep: {deep}\npriority: {priority}",
            regex = self.regex,
            priority = self.priority,
            deep = self.deep
        )
    }
}
