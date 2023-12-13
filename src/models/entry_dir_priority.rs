#[derive(Default, Clone)]
pub struct EntryDirPriority {
    pub regex: String,
    pub deep: usize,
    pub priority: usize,
    pub root: Option<String>,
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
