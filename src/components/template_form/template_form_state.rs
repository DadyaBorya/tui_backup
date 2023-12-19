#[derive(Default)]
pub struct TemplateFormState {
    pub name: String,
    pub is_edit: bool,
}

impl TemplateFormState {
    pub fn init() -> Self {
        TemplateFormState::default()
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
