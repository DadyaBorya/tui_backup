use regex::Regex;

pub fn empty(str: &String) -> Result<String, String> {
    if str.is_empty() { Err("empty".to_string()) } else { Ok(str.clone()) }
}

pub fn regex(str: &String) -> Result<String, String> {
    if let Err(err) = empty(str) {
        return Err(format!("Invalid value: {}", err));
    }

    match Regex::new(&str) {
        Ok(_) => Ok(str.as_str().to_string()),
        Err(_) => Err(format!("Invalid value: {}", str)),
    }
}

pub fn uszie(str: &String) -> Result<usize, String> {
    if let Err(err) = empty(str) {
        return Err(format!("Invalid value: {}", err));
    }

    match str.parse::<usize>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("Invalid value: {}", str)),
    }
}
