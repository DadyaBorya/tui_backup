use std::usize;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct EntryFileFilter {
    pub regex: String,
    pub content: String,
    pub deep: Option<usize>,
    pub root: String,
}

impl EntryFileFilter {
    pub fn row(&self) -> String {
        let mut deep_str = EntryFileFilter::get_deep(self.deep);

        if deep_str.is_empty() {
            deep_str = "*".to_string();
        }

        format!(
            "regex: {regex} deep: {deep}\n{content}",
            regex = self.regex,
            content = self.content,
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
