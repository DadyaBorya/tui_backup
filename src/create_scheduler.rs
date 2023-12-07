#[derive(Clone)]
pub struct CreateScheduler {
    pub name: String,
    pub clouds: String,
    pub protocols: String,
    pub cron: String,
    pub speed_limit: String,
}

impl CreateScheduler {
    pub fn new(
        name: String,
        clouds: String,
        protocols: String,
        cron: String,
        speed_limit: String
    ) -> Self {
        CreateScheduler { name, clouds, protocols, cron, speed_limit }
    }

    pub fn clear_inputs(&mut self) {
        self.name.clear();
        self.clouds.clear();
        self.protocols.clear();
        self.cron.clear();
        self.speed_limit.clear();
    }
}
