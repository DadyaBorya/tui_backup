#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct EntryDirFilePriority {
    pub regex: String,
    pub content: String,
    pub priority: usize,
    pub deep: Option<usize>,
    pub root: String,
}

impl EntryDirFilePriority {
    pub fn row(&self) -> String {
        let mut deep_str = EntryDirFilePriority::get_deep(self.deep);

        if deep_str.is_empty() {
            deep_str = "*".to_string();
        }

        format!(
            "regex: {regex} priority: {priority} deep: {deep}\n{content}",
            regex = self.regex,
            content = self.content,
            priority = self.priority,
            deep = deep_str
        )
    }

    pub fn get_deep(deep: Option<usize>) -> String {
        match deep {
            Some(value) => value.to_string(),
            None => "".to_string(),
        }
    }
}
