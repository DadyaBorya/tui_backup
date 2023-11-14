use std::fs;
use crate::file_service;

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;
use tui::style::Color;

#[derive(Debug, Clone)]
pub struct FileSystem {
    pub root_dir: Folder,
    pub current_path: String,
    pub rows: Vec<(Vec<String>, Color)>,
    pub history_index: Vec<usize>,
}

impl FileSystem {
    pub fn new() -> Result<Self, std::io::Error> {
        let items = file_service::get_root_system_items()?;
        let root = Folder::new("/".to_string(), "/".to_string(), false, items, true, None, "dir".to_string());

        Ok(FileSystem {
            root_dir: root,
            current_path: "/".to_string(),
            rows: vec![],
            history_index: vec![],
        })
    }

    fn find_folder_by_path_recursive<'a>(folder: &'a mut Folder, target_path: &String) -> Option<&'a mut Folder> {
        if folder.path.as_str() == target_path {
            return Some(folder);
        }

        for content in &mut folder.contents {
            match content {
                FileSystemItem::File_(_) => {}
                FileSystemItem::Folder_(subfolder) => {
                    if let Some(found_folder) =
                        FileSystem::find_folder_by_path_recursive(subfolder, target_path)
                    {
                        return Some(found_folder);
                    }
                }
            }
        }

        None
    }

    pub fn find_folder_by_path(&mut self, target_path: &String) -> Option<&mut Folder> {
        FileSystem::find_folder_by_path_recursive(&mut self.root_dir, target_path)
    }

    fn find_file_by_path_recursive<'a>(folder: &'a mut Folder, target_path: &String) -> Option<&'a mut File> {
        for content in &mut folder.contents {
            match content {
                FileSystemItem::File_(file) => {
                    if file.path.as_str() == target_path {
                        return Some(file);
                    }
                }
                FileSystemItem::Folder_(subfolder) => {
                    if let Some(found_folder) =
                        FileSystem::find_file_by_path_recursive(subfolder, target_path)
                    {
                        return Some(found_folder);
                    }
                }
            }
        }

        None
    }

    pub fn find_file_by_path(&mut self, target_path: &String) -> Option<&mut File> {
        FileSystem::find_file_by_path_recursive(&mut self.root_dir, target_path)
    }

    pub fn check_access(path: &String) -> bool {
        return match fs::metadata(path) {
            Ok(metadata) => {
                let permissions = metadata.permissions();
                #[cfg(target_os = "windows")]
                {
                    return permissions.readonly() == false;
                }

                #[cfg(target_os = "linux")]
                {
                    return permissions.mode() & 0o777 == 0o777;
                }
            }
            Err(_) => { false }
        };
    }

    pub fn set_rows_of_current_dir(&mut self) {
        let current_dir = self.find_folder_by_path(&self.current_path.clone());

        let mut items_string = vec![];

        if let Some(dir) = current_dir {
            for content in dir.contents.iter() {
                let color;
                let item = match content {
                    FileSystemItem::File_(item) => {
                        color = Color::Blue;
                        FileSystem::string_item(
                            item.name.to_owned(),
                            item.selected,
                            item.access,
                            item.size,
                            item.extension.to_owned(),
                        )
                    }
                    FileSystemItem::Folder_(item) => {
                        color = Color::Green;
                        FileSystem::string_item(
                            item.name.to_owned(),
                            item.selected,
                            item.access,
                            item.size,
                            item.extension.to_owned(),
                        )
                    }
                };
                let item_parts: Vec<String> = item.split("|").map(|s| s.to_owned()).collect();
                items_string.push((item_parts, color));
            }
            self.rows = items_string
        } else {
            self.rows = items_string
        }
    }

    pub fn string_item(name: String, selected: bool, access: bool, size: Option<u64>, extension: String) -> String {
        let selected = match selected {
            true => {
                "[x]"
            }
            false => {
                "[ ]"
            }
        };

        let access = match access {
            true => {
                "Yes"
            }
            false => {
                "No"
            }
        };

        let size = match size {
            None => { "undefined".to_string() }
            Some(size) => { size.to_string() }
        };

        format!("{}|{}|{}|{}|{}", selected, name, extension, size, access)
    }

    pub fn select(&mut self, index: usize) {
        let current_path = &self.current_path.clone();

        if let Some(dir) = self.find_folder_by_path(current_path) {
            for (i, item) in dir.contents.iter_mut().enumerate() {
                if i == index {
                    match item {
                        FileSystemItem::File_(file) => {
                            file.selected = !file.selected;
                        }
                        FileSystemItem::Folder_(folder) => {
                            folder.selected = !folder.selected;
                        }
                    }
                }
            }
        }
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
    pub size: Option<u64>,
    pub extension: String,
}

impl Folder {
    pub fn new(name: String, path: String, selected: bool, contents: Vec<FileSystemItem>, access: bool, size: Option<u64>, extension: String) -> Self {
        Folder {
            name,
            path,
            selected,
            contents,
            access,
            size,
            extension,
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub selected: bool,
    pub access: bool,
    pub size: Option<u64>,
    pub extension: String,
}

impl File {
    pub fn new(name: String, path: String, selected: bool, access: bool, size: Option<u64>, extension: String) -> Self {
        File {
            name,
            path,
            selected,
            access,
            size,
            extension,
        }
    }
}