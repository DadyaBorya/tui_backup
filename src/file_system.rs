use crate::file_service;
use std::cmp::Ordering;
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;
use tui::style::Color;
use crate::file_list_filter::{FileFilter, FolderFilter};

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
        let mut root = Folder::new("/".to_string(), "/".to_string(), false, items, "dir".to_string(), vec![], vec![]);
        root.sort_contents();

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


    pub fn set_rows_of_current_dir(&mut self) {
        let current_dir = self.find_folder_by_path(&self.current_path.clone());

        let mut items_string = vec![];

        if let Some(dir) = current_dir {
            for content in dir.contents.iter() {
                let color;
                let item = match content {
                    FileSystemItem::File_(item) => {
                        color = Color::Blue;
                        FileSystem::string_items(
                            item.name.to_owned(),
                            item.selected,
                            item.extension.to_owned(),
                        )
                    }
                    FileSystemItem::Folder_(item) => {
                        color = Color::Green;
                        FileSystem::string_items(
                            item.name.to_owned(),
                            item.selected,
                            item.extension.to_owned(),
                        )
                    }
                };
                items_string.push((item, color));
            }
            self.rows = items_string
        } else {
            self.rows = items_string
        }
    }

    pub fn string_items(name: String, selected: bool, extension: String) -> Vec<String> {
        let selected = match selected {
            true => {
                "[x]"
            }
            false => {
                "[ ]"
            }
        };

        vec![selected.to_string(), name, extension]
    }

    pub fn select(&mut self, index: usize) {
        let current_path = &self.current_path.clone();

        if let Some(dir) = self.find_folder_by_path(current_path) {
            for (i, item) in dir.contents.iter_mut().enumerate() {
                if i == index {
                    FileSystem::select_item(item);
                }
            }
        }
    }

    pub fn select_item(item: &mut FileSystemItem) {
        match item {
            FileSystemItem::File_(file) => {
                file.selected = !file.selected;
            }
            FileSystemItem::Folder_(folder) => {
                folder.selected = !folder.selected;
            }
        }
    }

    pub fn select_all(&mut self) {
        let current_path = &self.current_path.clone();
        if let Some(dir) = self.find_folder_by_path(current_path) {
            for item in dir.contents.iter_mut() {
                FileSystem::select_item(item);
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
    pub extension: String,
    pub file_filter_rules: Vec<FileFilter>,
    pub folder_filter_rules: Vec<FolderFilter>,
}

impl Folder {
    pub fn new(name: String, path: String, selected: bool, contents: Vec<FileSystemItem>, extension: String, file_filter_rules: Vec<FileFilter>, folder_filter_rules: Vec<FolderFilter>) -> Self {
        Folder {
            name,
            path,
            selected,
            contents,
            extension,
            folder_filter_rules,
            file_filter_rules
        }
    }

    pub fn sort_contents(&mut self) {
        self.contents.sort_by(|a, b| {
            match (a, b) {
                (FileSystemItem::Folder_(folder_a), FileSystemItem::Folder_(folder_b)) => {
                    folder_a.name.cmp(&folder_b.name)
                }
                (FileSystemItem::File_(file_a), FileSystemItem::File_(file_b)) => {
                    file_a.name.cmp(&file_b.name)
                }
                // Folders come before files
                (FileSystemItem::Folder_(_), FileSystemItem::File_(_)) => Ordering::Less,
                (FileSystemItem::File_(_), FileSystemItem::Folder_(_)) => Ordering::Greater,
            }
        });
    }

    pub fn add_existing_item(&mut self, item: FileSystemItem) {
        if !self.contents.iter().any(|existing_item| {
            match (existing_item, &item) {
                (FileSystemItem::File_(existing_file), FileSystemItem::File_(new_file)) => existing_file.name == new_file.name,
                (FileSystemItem::Folder_(existing_folder), FileSystemItem::Folder_(new_folder)) => existing_folder.name == new_folder.name,
                _ => false,
            }
        }) {
            self.contents.push(item);
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub selected: bool,
    pub extension: String,
}

impl File {
    pub fn new(name: String, path: String, selected: bool, extension: String) -> Self {
        File {
            name,
            path,
            selected,
            extension,
        }
    }
}