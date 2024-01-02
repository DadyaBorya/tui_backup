const HEADERS: [&'static str; 3] = ["File Explorer", "Templates", "Schedulers"];

pub struct TabState {
    pub headers: Vec<String>,
    pub index: usize,
}

impl TabState {
    pub fn init() -> Self {
        let headers = HEADERS.iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>();
        TabState { headers, index: 0 }
    }
}
