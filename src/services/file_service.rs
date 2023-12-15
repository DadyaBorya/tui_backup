use std::{ path::{ PathBuf, Path }, fs::{ self, OpenOptions }, io::Write };

use crate::models::dir_entry::DirEntry;

#[cfg(target_os = "windows")]
pub fn root() -> Result<Vec<DirEntry>, std::io::Error> {
    let mut entries = Vec::new();

    for drive_value in b'A'..=b'Z' {
        let drive_letter = char::from(drive_value);
        let path_str = format!(r"{}:\", drive_letter);
        let path = PathBuf::from(path_str);

        if path.is_dir() {
            let mut entry = DirEntry::default();
            entry.path = path;
            entries.push(entry);
        }
    }

    Ok(entries)
}

#[cfg(target_os = "linux")]
pub fn root() -> Result<Vec<DirEntry>, std::io::Error> {
    entries(Path::new("/"))
}

pub fn entries(path: &Path) -> Result<Vec<DirEntry>, std::io::Error> {
    let entries = fs::read_dir(path)?;
    let mut dir_entries = vec![];

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let mut dir_entry = DirEntry::default();
        dir_entry.path = path;

        dir_entries.push(dir_entry);
    }

    Ok(dir_entries)
}

pub fn save(path: &Path, content: String) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
