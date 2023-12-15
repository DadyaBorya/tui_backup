#[derive(Default)]
pub struct CreateTemplateFormState {
    pub name: String,
}

impl CreateTemplateFormState {
    pub fn init() -> Self {
        CreateTemplateFormState::default()
    }

    pub fn validate(&self) -> Result<String, Vec<String>> {
        let mut errors = vec![];
        match self.name.is_empty() {
            false => {}
            true => errors.push(format!("Name field: [{}]", "empty")),
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(self.name.clone())
    }

    pub fn clear(&mut self) {
        self.name.clear();
    }
}
