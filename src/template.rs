#[derive(Debug)]
pub struct Template {
    pub form_name: String,
}

impl Template {
    pub fn new(name: String) -> Template {
        Template {
            form_name: name,
        }
    }

    pub fn clear_inputs(&mut self) {
        self.form_name.clear();
    } 
}
