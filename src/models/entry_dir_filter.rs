#[derive(Default, Clone)]
pub struct EntryDirFilter {
    pub regex: String,
    pub deep: usize,
    pub root: Option<String>,
}

impl EntryDirFilter {
    pub fn row(&self) -> String {
        format!("regex: {regex} deep: {deep}", regex = self.regex, deep = self.deep)
    }
}
