use serde::{ Serialize, Deserialize };

#[derive(Clone, Serialize, Deserialize)]
pub enum Cloud {
    Mega,
    Dropbox,
    GoogleDrive,
}

impl Cloud {
    pub fn from_str(str: &str) -> Option<Cloud> {
        match str {
            "Mega" => Some(Cloud::Mega),
            "Dropbox" => Some(Cloud::Dropbox),
            "GoogleDrive" => Some(Cloud::GoogleDrive),
            _ => { None }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Cloud::Mega => "Mega".to_string(),
            Cloud::Dropbox => "Dropbox".to_string(),
            Cloud::GoogleDrive => "GoogleDrive".to_string(),
        }
    }

    pub fn vec_to_string(vec: &Vec<Cloud>) -> String {
        vec.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Protocol {
    Https,
    Webdav,
}

impl Protocol {
    pub fn from_str(str: &str) -> Option<Protocol> {
        match str {
            "Https" => Some(Protocol::Https),
            "Webdav" => Some(Protocol::Webdav),
            _ => { None }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Protocol::Https => "Https".to_string(),
            Protocol::Webdav => "Webdav".to_string(),
        }
    }

    pub fn vec_to_string(vec: &Vec<Protocol>) -> String {
        vec.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Scheduler {
    pub name: String,
    pub id_process: String,
    pub clouds: Vec<Cloud>,
    pub protocols: Vec<Protocol>,
    pub cron: String,
    pub speed_limit: usize,
    pub template_path: String,
    pub out_path: String,
}

impl Scheduler {
    pub fn new(
        name: String,
        id_process: String,
        clouds: Vec<Cloud>,
        protocols: Vec<Protocol>,
        cron: String,
        speed_limit: usize,
        template_path: String,
        out_path: String
    ) -> Self {
        Scheduler {
            name,
            id_process,
            clouds,
            protocols,
            cron,
            speed_limit,
            template_path,
            out_path,
        }
    }

    pub fn get_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
