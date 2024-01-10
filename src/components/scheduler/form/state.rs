use std::collections::HashMap;

use tui::widgets::ListState;

use crate::{
    models::scheduler::{ Cloud, Protocol, Scheduler },
    utils::{ list_utils, validator },
    application::mode::SchedulerForm,
};

const CRON_TEMPLATES: [(&'static str, &'static str); 20] = [
    ("every minute", "* * * * *"),
    ("every 5 minutes", "*/5 * * * *"),
    ("30 minutes past the hour", "30 * * * *"),
    ("midnight", "0 0 * * *"),
    ("3 AM on Sundays", "0 3 * * 0"),
    ("8 AM on weekdays", "0 8 * * 1-5"),
    ("2 PM on the first day of the month", "0 14 1 * *"),
    ("every 15 minutes", "15 * * * *"),
    ("midnight on Sundays and Wednesdays", "0 0 * * 0,3"),
    ("every 2 hours", "0 */2 * * *"),
    ("every 15 minutes", "*/15 * * * *"),
    ("6 AM on Mondays", "0 6 * * 1"),
    ("4:30 AM", "30 4 * * *"),
    ("8 PM on Fridays", "0 20 * * 5"),
    ("2 AM on the 12th day of the month", "0 12 2 * *"),
    ("every 30 minutes", "*/30 * * * *"),
    ("every 3 hours", "0 */3 * * *"),
    ("9 AM on Thursdays", "0 9 * * 4"),
    ("3:45 PM", "45 15 * * *"),
    ("11:59 PM on Sundays", "59 23 * * 0"),
];

#[derive(Default)]
pub struct SchedulerFormState {
    pub root: String,
    pub name: String,
    pub cron: String,
    pub speed: String,
    pub cron_templates: Vec<String>,
    pub cron_list_state: ListState,
    pub clouds_protocols: Vec<((Cloud, bool), Vec<(Protocol, bool)>)>,
    pub cloud_list_state: ListState,
    pub protocol_list_state: ListState,
    pub is_edit: bool
}

impl SchedulerFormState {
    pub fn init() -> Self {
        let mut state = SchedulerFormState::default();
        state.cron_templates = CRON_TEMPLATES.iter()
            .map(|(k, _): &(&str, &str)| k.to_string())
            .collect::<Vec<String>>();
        state.fill_cloud_protocols();
        state
    }

    pub fn clouds(&self) -> Vec<String> {
        self.clouds_protocols
            .iter()
            .map(|cp| format!("[{}] {:?}", if cp.0.1 { 'x' } else { ' ' }, cp.0.0))
            .collect()
    }

    pub fn protocols(&mut self) -> Vec<String> {
        if let Some((_, v)) = self.selected_cloud() {
            return v
                .iter()
                .map(|r| format!("[{}] {:?}", if r.1 { 'x' } else { ' ' }, r.0))
                .collect();
        }

        vec![]
    }

    pub fn fill_cloud_protocols(&mut self) {
        self.clouds_protocols = vec![];
        for cloud in Cloud::list() {
            let protocols = cloud
                .protocols()
                .iter()
                .map(|protocol| (protocol.clone(), false))
                .collect::<Vec<(Protocol, bool)>>();

            self.clouds_protocols.push(((cloud, false), protocols));
        }
    }

    pub fn clear(&mut self) {
        self.name.clear();
        self.speed.clear();
        self.cron.clear();
        self.cron_list_state.select(None);
        self.cloud_list_state.select(None);
        self.protocol_list_state.select(None);
        self.fill_cloud_protocols();
    }

    pub fn selected_cloud(&mut self) -> Option<&mut ((Cloud, bool), Vec<(Protocol, bool)>)> {
        if let Some(index) = self.cloud_list_state.selected() {
            return self.clouds_protocols.get_mut(index);
        }

        None
    }

    pub fn selected_protocols(&mut self) -> Option<&mut (Protocol, bool)> {
        if
            let (Some(index), Some(cloud)) = (
                self.protocol_list_state.selected(),
                self.selected_cloud(),
            )
        {
            return Some(&mut cloud.1[index]);
        }

        None
    }

    pub fn init_index_list(&mut self, mode: &SchedulerForm) {
        match mode {
            SchedulerForm::Cron => {
                let len = self.cron_templates.len();
                list_utils::init_index_table(&mut self.cron_list_state, len);
            }
            SchedulerForm::Cloud => {
                let len = self.clouds_protocols.len();
                list_utils::init_index_table(&mut self.cloud_list_state, len);
            }
            SchedulerForm::Protocol => {
                let len = self.protocols().len();
                list_utils::init_index_table(&mut self.protocol_list_state, len);
            }
            _ => {}
        }
    }

    pub fn selected_cron(&self) -> Option<(String, String)> {
        if let Some(index) = self.cron_list_state.selected() {
            if let Some((k, v)) = CRON_TEMPLATES.get(index) {
                return Some((k.to_string(), v.to_string()));
            }
        }

        None
    }

    pub fn validate(&self) -> Result<Scheduler, Vec<String>> {
        let mut errors = vec![];
        let mut scheduler = Scheduler::default();

        match validator::is_empty(&self.name) {
            Ok(value) => {
                scheduler.name = value;
            }
            Err(error) => errors.push(format!("Name field: [{}]", error)),
        }

        match validator::uszie(&self.speed) {
            Ok(value) => {
                scheduler.speed = value;
            }
            Err(error) => errors.push(format!("Speed limit field: [{}]", error)),
        }

        match validator::cron(&self.cron) {
            Ok(value) => {
                scheduler.cron = value;
            }
            Err(error) => errors.push(format!("Cron field: [{}]", error)),
        }
        let mut clouds = HashMap::new();

        for (cloud, protocols) in self.clouds_protocols.iter() {
            if !cloud.1 {
                continue;
            }

            let mut selected_protocols = vec![];

            for protocol in protocols {
                if protocol.1 {
                    selected_protocols.push(protocol.0);
                }
            }

            if !selected_protocols.is_empty() {
                clouds.insert(cloud.0, selected_protocols);
            }
        }

        match clouds.keys().len() == 0 {
            true =>
                errors.push(String::from("Clouds field: [Clouds and Protocols were not selected]")),
            false => {
                scheduler.clouds = clouds;
            }
        }

        scheduler.root = self.root.clone();

        match errors.is_empty() {
            true => Ok(scheduler),
            false => Err(errors),
        }
    }
}
