use std::path::PathBuf;

use serde::{ Serialize, Deserialize };

use crate::services::file_service;

const PATH: &'static str = "config.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct AppPaths {
    pub templates: String,
    pub schedulers: String,
    pub watcher_backup: String,
    pub config_path: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub paths: AppPaths,
}

impl Config {
    pub fn init() -> Result<Config, std::io::Error> {
        let contents = file_service::read_file(&PathBuf::from(PATH))?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
