#[derive(Default, Clone)]
pub struct EntryDirFilter {
    pub regex: String,
    pub deep: usize,
}

impl EntryDirFilter {
    pub fn row(&self) -> String {
        format!(
            "regex: {regex} deep: {deep}",
            regex = self.regex,
            deep = self.deep
        )
    }
}
