#[derive(Default, Clone)]
pub struct EntryDirFilePriority {
    pub regex: String,
    pub content: String,
    pub priority: usize,
    pub deep: usize
}

impl EntryDirFilePriority {
    pub fn row(&self) -> String {
        format!(
            "regex: {regex} priority: {priority} deep: {deep}\n{content}",
            regex = self.regex,
            content = self.content,
            priority = self.priority,
            deep = self.deep
        )
    }
}
