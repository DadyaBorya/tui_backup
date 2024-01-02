use std::collections::HashMap;

use serde::{ Serialize, Deserialize };
use tui::style::Color;

const ROW_COLOR: Color = Color::White;

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
    pub root: String,
}

impl Scheduler {
    pub fn row(&self) -> (Vec<String>, Color) {
        let clouds = self.clouds
            .iter()
            .map(|(k, v)| {
                let protocols = v
                    .iter()
                    .map(|p| format!("{:?}", p))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("{:?} [{}]", k, protocols)
            })
            .collect::<Vec<String>>()
            .join("\n");

        (
            vec![
                self.name.clone(),
                self.speed.to_string(),
                self.cron.clone(),
                clouds,
                self.root.clone()
            ],
            ROW_COLOR,
        )
    }
}
