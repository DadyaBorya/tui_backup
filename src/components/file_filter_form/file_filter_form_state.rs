use crate::{ utils::validator, models::entry_file_filter::EntryFileFilter };

#[derive(Default)]
pub struct FileFilterFormState {
    pub regex: String,
    pub deep: String,
    pub content: String,
}

impl FileFilterFormState {
    pub fn init() -> Self {
        FileFilterFormState::default()
    }

    pub fn clear(&mut self) {
        self.regex.clear();
        self.deep.clear();
        self.content.clear();
    }

    pub fn validate(&self) -> Result<EntryFileFilter, Vec<String>> {
        let mut errors = vec![];
        let mut filter = EntryFileFilter::default();

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
