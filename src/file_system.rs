use std::fs;
use crate::file_service;

#[derive(Debug, Clone)]
pub struct FileSystem {
    pub root_dir: Folder,
}

impl FileSystem {
    pub fn new() -> Self {
        let items = file_service::get_root_system_items();
        let root = Folder::new("/".to_string(), "/".to_string(), false, items, true, Option::None);

        FileSystem {
            root_dir: root
        }
    }

    pub fn check_access(path: &String) -> bool {
        return match fs::metadata(path) {
            Ok(metadata) => {
                let permissions = metadata.permissions();
                #[cfg(target_os = "windows")]
                {
                    return permissions.readonly() == false
                }

                #[cfg(target_os = "linux")]
                {
                    return permissions.mode() & 0o777 == 0o777
                }

            }
            Err(_) => { false }
        };
    }
}

#[derive(Debug, Clone)]
pub enum FileSystemItem {
    File_(File),
    Folder_(Folder),
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub path: String,
    pub selected: bool,
    pub contents: Vec<FileSystemItem>,
    pub access: bool,
    pub size: Option<u64>
}

impl Folder {
    pub fn new(name: String, path: String, selected: bool, contents: Vec<FileSystemItem>, access: bool, size: Option<u64>) -> Self {
        Folder {
            name,
            path,
            selected,
            contents,
            access,
            size
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub selected: bool,
    pub access: bool,
    pub size: Option<u64>
}

impl File {
    pub fn new(name: String, path: String, selected: bool, access: bool, size: Option<u64>) -> Self {
        File {
            name,
            path,
            selected,
            access,
            size
        }
    }
}