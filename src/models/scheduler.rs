use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Protocol {
    Https,
    Http,
    Webdav,
}

impl Protocol {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Cloud {
    Mega,
    GoogleDrive,
}

impl Cloud {
    pub fn protocols(&self) -> Vec<Protocol> {
        match self {
            Cloud::Mega => vec![Protocol::Https, Protocol::Http],
            Cloud::GoogleDrive => vec![Protocol::Https, Protocol::Http, Protocol::Webdav],
        }
    }

    pub fn list() -> Vec<Cloud> {
        vec![Cloud::Mega, Cloud::GoogleDrive]
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Scheduler {
    pub name: String,
    pub cron: String,
    pub speed: usize,
    pub clouds: HashMap<Cloud, Vec<Protocol>>,
    pub root: String
}
