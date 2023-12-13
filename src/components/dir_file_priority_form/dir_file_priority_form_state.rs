use crate::{ models::entry_dir_file_priority::EntryDirFilePriority, utils::validator };

#[derive(Default)]
pub struct DirFilePriorityFormState {
    pub regex: String,
    pub deep: String,
    pub priority: String,
    pub content: String,
}

impl DirFilePriorityFormState {
    pub fn init() -> Self {
        DirFilePriorityFormState::default()
    }

    pub fn clear(&mut self) {
        self.regex.clear();
        self.deep.clear();
        self.priority.clear();
        self.content.clear();
    }

    pub fn validate(&self) -> Result<EntryDirFilePriority, Vec<String>> {
        let mut errors = vec![];
        let mut filter = EntryDirFilePriority::default();

        match validator::regex(&self.regex) {
            Ok(value) => {
                filter.regex = value;
            }
            Err(error) => errors.push(format!("Regex field: [{}]", error)),
        }

        match validator::uszie(&self.deep) {
            Ok(value) => {
                filter.deep = value;
            }
            Err(error) => errors.push(format!("Deep field: [{}]", error)),
        }

        match validator::uszie(&self.priority) {
            Ok(value) => {
                filter.priority = value;
            }
            Err(error) => errors.push(format!("Priority field: [{}]", error)),
        }

        match validator::regex(&self.content) {
            Ok(value) => {
                filter.content = value;
            }
            Err(error) => errors.push(format!("Content field: [{}]", error)),
        }

        match errors.is_empty() {
            true => Ok(filter),
            false => Err(errors),
        }
    }
}
