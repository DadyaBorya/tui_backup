#[derive(Default, Clone, PartialEq, Eq)]
pub struct EntryFileFilter {
    pub regex: String,
    pub content: String,
    pub deep: usize,
}

impl EntryFileFilter {
    pub fn row(&self) -> String {
        format!(
            "regex: {regex} deep: {deep}\n{content}",
            regex = self.regex,
            content = self.content,
            deep = self.deep
        )
    }
}
