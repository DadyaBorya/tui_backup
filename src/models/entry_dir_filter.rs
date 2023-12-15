#[derive(Default, Clone, PartialEq, Eq)]
pub struct EntryDirFilter {
    pub regex: String,
    pub deep: usize,
    pub root: String,
}

impl EntryDirFilter {
    pub fn row(&self) -> String {
        format!("regex: {regex} deep: {deep}", regex = self.regex, deep = self.deep)
    }
}
