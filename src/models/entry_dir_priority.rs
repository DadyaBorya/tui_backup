#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct EntryDirPriority {
    pub regex: String,
    pub deep: Option<usize>,
    pub priority: usize,
    pub root: String,
}

impl EntryDirPriority {
    pub fn row(&self) -> String {
        let mut deep_str = EntryDirPriority::get_deep(self.deep);

        if deep_str.is_empty() {
            deep_str = "*".to_string();
        }

        format!(
            "regex: {regex} deep: {deep}\npriority: {priority}",
            regex = self.regex,
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
