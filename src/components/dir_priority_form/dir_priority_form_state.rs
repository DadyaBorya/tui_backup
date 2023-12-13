use crate::{models::entry_dir_priority::EntryDirPriority, utils::validator};

#[derive(Default)]
pub struct DirPriorityFormState {
    pub regex: String,
    pub priority: String,
    pub deep: String,
}

impl DirPriorityFormState {
    pub fn init() -> Self {
        DirPriorityFormState::default()
    }

    pub fn clear(&mut self) {
        self.regex.clear();
        self.deep.clear();
        self.priority.clear();
    }

    pub fn validate(&self) -> Result<EntryDirPriority, Vec<String>> {
        let mut errors = vec![];
        let mut filter = EntryDirPriority::default();

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

        match errors.is_empty() {
            true => Ok(filter),
            false => Err(errors),
        }
    }
}
