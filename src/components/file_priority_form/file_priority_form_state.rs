use crate::{ models::entry_file_priority::EntryFilePriority, utils::validator };

#[derive(Default)]
pub struct FilePriorityFormState {
    pub content: String,
    pub priority: String,
}

impl FilePriorityFormState {
    pub fn init() -> Self {
        FilePriorityFormState::default()
    }

    pub fn clear(&mut self) {
        self.priority.clear();
        self.content.clear();
    }

    pub fn validate(&self) -> Result<EntryFilePriority, Vec<String>> {
        let mut errors = vec![];
        let mut filter = EntryFilePriority::default();

        match validator::uszie(&self.priority) {
            Ok(value) => {
                filter.priority = value;
            }
            Err(error) => errors.push(format!("Priority field: [{}]", error)),
        }
        
        if !self.content.is_empty() {
            match validator::regex(&self.content) {
                Ok(value) => {
                    filter.content = value;
                }
                Err(error) => errors.push(format!("Content field: [{}]", error)),
            }
        }

        match errors.is_empty() {
            true => Ok(filter),
            false => Err(errors),
        }
    }
}
