use regex::Regex;

fn empty(str: &String) -> Result<String, String> {
    if str.is_empty() { Err("empty".to_string()) } else { Ok(str.clone()) }
}

pub fn is_empty(str: &String) -> Result<String, String> {
    match empty(str) {
        Ok(_) => Ok(str.as_str().to_string()),
        Err(err) => Err(format!("Invalid value: {}", err)),
    }
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

pub fn cron(str: &String) -> Result<String, String> {
    if let Err(err) = empty(str) {
        return Err(format!("Invalid value: {}", err));
    }

    let regex = Regex::new(
        r"^((((\d+,)+\d+|(\d+(\/|-|#)\d+)|\d+L?|\*(\/\d+)?|L(-\d+)?|\?|[A-Z]{3}(-[A-Z]{3})?) ?){5,7})$"
    ).unwrap();

    match regex.is_match(&str) {
        true => Ok(str.as_str().to_string()),
        false => Err(format!("Invalid value: {}", str)),
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

pub fn ufloat(str: &String) -> Result<f32, String> {
    if let Err(err) = empty(str) {
        return Err(format!("Invalid value: {}", err));
    }

    match str.parse::<f32>() {
        Ok(value) => {
            if value <= 0.0 {
                return Err(format!("Invalid value: {}", str));
            }

            Ok(value)
        },
        Err(_) => Err(format!("Invalid value: {}", str)),
    }
}
