use std::fs;
use std::path::{Path, PathBuf};

use crate::file_system::{File, FileSystem, FileSystemItem, Folder};

#[cfg(target_os = "windows")]
pub fn get_root_system_items() -> Result<Vec<FileSystemItem>, std::io::Error> {
    let mut system_items = Vec::new();

    for drive_value in b'A'..=b'Z' {
        let drive_letter = char::from(drive_value);
        let path = format!("{}:/", drive_letter);


        match PathBuf::from(&path).is_dir() {
            true => {
                let access = FileSystem::check_access(&path);
                let item = FileSystemItem::Folder_(Folder::new(
                    format!("{}:", drive_letter),
                    path,
                    false,
                    vec![],
                    access, None, "dir".to_string(),
                ));
                system_items.push(item);
            }
            _ => {}
        };
    }
    Ok(system_items)
}

#[cfg(target_os = "linux")]
pub fn get_root_system_items() -> Result<Vec<FileSystemItem>, std::io::Error> {
    return get_system_items_from_path("/".to_string());
}

pub fn get_system_items_from_path(path: String) -> Result<Vec<FileSystemItem>, std::io::Error> {
    let entries = fs::read_dir(path)?;

    let mut system_items = Vec::new();

    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        let mut path_string = path.to_str().unwrap_or_default().to_string();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let access = FileSystem::check_access(&path_string);


        let item = match entry.path().is_dir() {
            true => FileSystemItem::Folder_(Folder {
                name: file_name.clone(),
                path: normalize_path(&mut path_string),
                selected: false,
                contents: vec![],
                access,
                size: None,
                // size: get_item_size(&path_string)
                extension: "dir".to_string(),
            }),
            false => {
                let extension = match Path::new(&file_name).extension() {
                    None => {
                        "unknown".to_string()
                    }
                    Some(exten) => {
                        exten.to_str().unwrap().to_string()
                    }
                };

                FileSystemItem::File_(File {
                    name: file_name.clone(),
                    path: normalize_path(&mut path_string),
                    selected: false,
                    access,
                    size: None,
                    // size: get_item_size(&path_string),
                    extension,
                })
            }
        };
        system_items.push(item);
    }


    Ok(system_items)
}

pub fn normalize_path(path: &mut String) -> String {
    path.replace("\\", "/")
        .replace("//", "/")
}

// fn get_item_size(path: &String) -> Option<u64> {
//     let size = get_size(path);
//
//     match size {
//         Ok(size) => { Some(size) }
//         Err(_) => { None }
//     }
// }