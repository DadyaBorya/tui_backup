use crate::file_service;
use std::cmp::Ordering;
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
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

    pub fn set_rows_of_current_dir(&mut self) {
        let current_dir = self.root_dir.find_folder_mut(&self.current_path.clone());

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

        if let Some(dir) = self.root_dir.find_folder_mut(current_path) {
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

        if let Some(dir) = self.root_dir.find_folder_mut(current_path) {
            for item in dir.contents.iter_mut() {
                FileSystem::select_item(item);
            }
        }
    }
    pub fn get_current_folder(&mut self) -> Option<&mut Folder> {
        self.root_dir.find_folder_mut(&self.current_path)
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
        Folder { name, path, selected, contents, extension, folder_filter_rules, file_filter_rules }
    }

    pub fn add_children_to_folder(&mut self) -> Result<(), std::io::Error> {
        let content = file_service::get_system_items_from_path(self.path.clone())?;

        self.add_existing_items(content.clone());
        self.delete_not_existing_items(content);
        self.sort_contents();
        Ok(())
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
                (FileSystemItem::Folder_(_), FileSystemItem::File_(_)) => Ordering::Less,
                (FileSystemItem::File_(_), FileSystemItem::Folder_(_)) => Ordering::Greater,
            }
        });
    }
    pub fn find_folder_mut(&mut self, path: &String) -> Option<&mut Folder> {
        if &self.path == path {
            return Some(self);
        }

        for content in self.contents.iter_mut() {
            if let FileSystemItem::Folder_(folder) = content {
                if &folder.path == path {
                    return Some(folder);
                } else {
                    if let Some(found_folder) = folder.find_folder_mut(path) {
                        return Some(found_folder);
                    }
                }
            }
        }

        None
    }
    pub fn find_file_mut(&mut self, path: &String) -> Option<&mut File> {
        let folder_path = Folder::get_folder_path_from_path(path);

        if let Some(path) = folder_path {
            if let Some(folder) = self.find_folder_mut(&path.to_string()) {
                if let Some(file) = folder.contents.iter_mut().find(|item| {
                    if let FileSystemItem::File_(file) = item {
                        file.name == path
                    } else {
                        false
                    }
                }) {
                    if let FileSystemItem::File_(file) = file {
                        return Some(file);
                    }
                }
            }
        }

        None
    }
    pub fn find_folder_mut_in_content(&mut self, index: usize) -> Option<&mut Folder> {
        if index >= self.contents.len() {
            return None;
        }

        let item = &mut self.contents[index];

        if let FileSystemItem::Folder_(folder) = item {
            return Some(folder);
        }

        None
    }
    pub fn find_file_mut_in_content(&mut self, index: usize) -> Option<&mut File> {
        if index >= self.contents.len() {
            return None;
        }

        let item = &mut self.contents[index];

        if let FileSystemItem::File_(file) = item {
            return Some(file);
        }

        None
    }
    pub fn get_folder_path_from_path(file_path: &String) -> Option<&str> {
        let path = Path::new(file_path);
        path.parent().and_then(|parent| parent.to_str())
    }
    pub fn add_existing_items(&mut self, items: Vec<FileSystemItem>) {
        items.iter().for_each(|item| self.add_existing_item(item.clone()))
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
    pub fn delete_not_existing_items(&mut self, items: Vec<FileSystemItem>) {
        let mut contents = Vec::new();

        for content_item in &self.contents {
            if items.iter().any(|item| match (content_item, item) {
                (FileSystemItem::File_(file1), FileSystemItem::File_(file2)) => file1.path == file2.path,
                (FileSystemItem::Folder_(folder1), FileSystemItem::Folder_(folder2)) => folder1.path == folder2.path,
                _ => false,
            }) {
                contents.push(content_item.clone());
            }
        }

        self.contents = contents;
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