use std::path::PathBuf;

use serde::{ Serialize, Deserialize };

use crate::services::file_service;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppPaths {
    pub templates: String,
    pub schedulers: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    paths: AppPaths,
}

impl Config {
    pub fn init() -> Result<Config, std::io::Error> {
        let contents = file_service::read_file(&PathBuf::from("config.json"))?;
        let config: Config = serde_json::from_str(&contents)?;

        Ok(config)
    }
}
