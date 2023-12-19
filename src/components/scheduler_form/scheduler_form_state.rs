#[derive(Default)]
pub struct SchedulerFormState {
    pub name: String,
    pub cron: String,
    pub speed: String,
}

impl SchedulerFormState {
    pub fn clear(&mut self) {
        self.name.clear();
        self.speed.clear();
    }

    pub fn init() -> Self {
        SchedulerFormState::default()
    }
}
