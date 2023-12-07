#[derive(Debug)]
pub struct CreateTemplate {
    pub form_name: String,
}

impl CreateTemplate {
    pub fn new(name: String) -> CreateTemplate {
        CreateTemplate {
            form_name: name,
        }
    }

    pub fn clear_inputs(&mut self) {
        self.form_name.clear();
    }
}
