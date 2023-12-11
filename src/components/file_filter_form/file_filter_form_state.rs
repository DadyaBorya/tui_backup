use crate::utils::validator;

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

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = vec![];

        if let Err(error) = validator::regex(&self.regex) {
            errors.push(format!("Regex field: [{}]", error));
        }

        if let Err(error) = validator::uszie(&self.deep) {
            errors.push(format!("Deep field: [{}]", error));
        }

        if let Err(error) = validator::regex(&self.content) {
            errors.push(format!("Content field: [{}]", error));
        }

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }
}
