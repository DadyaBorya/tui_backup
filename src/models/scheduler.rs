#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Protocol {
    Https,
    Http,
    Webdav,
}

impl Protocol {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
