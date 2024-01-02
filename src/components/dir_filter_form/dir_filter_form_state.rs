use crate::{ models::entry_dir_filter::EntryDirFilter, utils::validator };

#[derive(Default)]
pub struct DirFilterFormState {
    pub regex: String,
    pub deep: String,
}

impl DirFilterFormState {
    pub fn init() -> Self {
        DirFilterFormState::default()
    }

    pub fn clear(&mut self) {
        self.regex.clear();
        self.deep.clear();
    }

    pub fn validate(&self) -> Result<EntryDirFilter, Vec<String>> {
        let mut errors = vec![];
        let mut filter = EntryDirFilter::default();

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

        match errors.is_empty() {
            true => Ok(filter),
            false => Err(errors),
        }
    }
}
