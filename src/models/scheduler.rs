use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Protocol {
    Https,
    Http,
    Webdav,
}

impl Protocol {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Cloud {
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

#[derive(Default)]
struct Scheduler {
    pub clouds: HashMap<Cloud, Vec<Protocol>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let clouds = Cloud::list()
            .iter()
            .fold(HashMap::new(), |mut acc, cloud| {
                acc.insert(*cloud, cloud.protocols());
                acc
            });
        Scheduler { clouds }
    }
}
