use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{ Path, PathBuf };

use crate::file_system::{ File, FileSystemItem, Folder };
use crate::scheduler::Scheduler;

#[cfg(target_os = "windows")]
pub fn get_root_system_items() -> Result<Vec<FileSystemItem>, std::io::Error> {
    let mut system_items = Vec::new();
    for drive_value in b'A'..=b'Z' {
        let drive_letter = char::from(drive_value);
        let path = format!("{}:/", drive_letter);
        match PathBuf::from(&path).is_dir() {
            true => {
                let item = FileSystemItem::Folder_(
                    Folder::new(
                        format!("{}:", drive_letter),
                        path,
                        false,
                        vec![],
                        "dir".to_string(),
                        vec![],
                        vec![],
                        vec![],
                        vec![]
                    )
                );
                system_items.push(item);
            }
            _ => {}
        };
    }
    Ok(system_items)
}

#[cfg(target_os = "linux")]
pub fn get_root_system_items() -> Result<Vec<FileSystemItem>, std::io::Error> {
    return get_system_items_from_path("/");
}

pub fn get_file_content(path: &String) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn save_content(path: &str, content: String) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn save_template(folder: &Folder, path: &str) -> Result<(), std::io::Error> {
    if !Path::new("templates").is_dir() {
        fs::create_dir("templates")?;
    }

    let json = folder.get_json();
    save_content(path, json)?;

    Ok(())
}

pub fn save_scheduler(scheduler: &Scheduler, path: &str) -> Result<(), std::io::Error> {
    if !Path::new("schedulers").is_dir() {
        fs::create_dir("schedulers")?;
    }

    let json = scheduler.get_json();
    save_content(path, json)?;

    Ok(())
}

pub fn get_system_items_from_path(path: &str) -> Result<Vec<FileSystemItem>, std::io::Error> {
    let entries = fs::read_dir(path)?;

    let mut system_items = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let mut path_string = path.to_str().unwrap_or_default().to_string();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        let item = match entry.path().is_dir() {
            true =>
                FileSystemItem::Folder_(
                    Folder::new(
                        file_name.clone(),
                        normalize_path(&mut path_string),
                        false,
                        vec![],
                        // size: get_item_size(&path_string)
                        "dir".to_string(),
                        vec![],
                        vec![],
                        vec![],
                        vec![]
                    )
                ),
            false => {
                let extension = match Path::new(&file_name).extension() {
                    None => { "unknown".to_string() }
                    Some(exten) => { exten.to_str().unwrap().to_string() }
                };

                FileSystemItem::File_(
                    File::new(
                        file_name.clone(),
                        normalize_path(&mut path_string),
                        false,
                        // size: get_item_size(&path_string),
                        extension,
                        vec![]
                    )
                )
            }
        };
        system_items.push(item);
    }

    Ok(system_items)
}

pub fn remove_file(path: &str) -> Result<(), std::io::Error> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn normalize_path(path: &mut String) -> String {
    path.replace("\\", "/").replace("//", "/")
}
