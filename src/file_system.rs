use crate::file_service;
use std::cmp::Ordering;
use std::collections::HashSet;
use regex::Regex;
use serde::{ Serialize, Deserialize };
use tui::style::Color;
use crate::file_item_list_filter::{ FileFolderFilter, FolderFilter };
use crate::file_item_list_priority::{ FileFolderPriority, FilePriority, FolderPriority };
use rayon::prelude::*;

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
        let mut root = Folder::new(
            "/".to_string(),
            "/".to_string(),
            false,
            items,
            "dir".to_string(),
            vec![],
            vec![],
            vec![],
            vec![]
        );
        root.sort_contents();

        let mut file_system = FileSystem {
            root_dir: root,
            current_path: "/".to_string(),
            rows: vec![],
            history_index: vec![],
        };

        file_system.set_rows_of_current_dir();

        Ok(file_system)
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
                            item.extension.to_owned()
                        )
                    }
                    FileSystemItem::Folder_(item) => {
                        color = Color::Green;
                        FileSystem::string_items(
                            item.name.to_owned(),
                            item.selected,
                            item.extension.to_owned()
                        )
                    }
                };
                items_string.push((item, color));
            }
            self.rows = items_string;
        } else {
            self.rows = items_string;
        }
    }
    pub fn string_items(name: String, selected: bool, extension: String) -> Vec<String> {
        let selected = match selected {
            true => { "[x]" }
            false => { "[ ]" }
        };

        vec![selected.to_string(), name, extension]
    }
    pub fn select(&mut self, index: usize) {
        let current_path = &self.current_path.clone();

        if let Some(dir) = self.root_dir.find_folder_mut(current_path) {
            for (i, item) in dir.contents.iter_mut().enumerate() {
                if i == index {
                    match item {
                        FileSystemItem::File_(file) => {
                            file.selected = !file.selected;
                        }
                        FileSystemItem::Folder_(folder) => {
                            if let Ok(_) = folder.add_children_to_folder() {
                                let bool = !folder.selected;

                                folder.selected = bool;

                                for item in folder.contents.iter_mut() {
                                    FileSystem::select_item_bool(item, bool);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn select_item_bool(item: &mut FileSystemItem, bool: bool) {
        match item {
            FileSystemItem::File_(file) => {
                file.selected = bool;
            }
            FileSystemItem::Folder_(folder) => {
                folder.selected = bool;
            }
        }
    }

    pub fn select_all(&mut self) {
        let current_path = &self.current_path.clone();

        if let Some(folder) = self.root_dir.find_folder_mut(current_path) {
            for item in folder.contents.iter_mut() {
                if let FileSystemItem::Folder_(folder) = item {
                    if let Ok(_) = folder.add_children_to_folder() {
                        let bool = !folder.selected;

                        folder.selected = bool;

                        for item in folder.contents.iter_mut() {
                            FileSystem::select_item_bool(item, bool);
                        }
                    }
                }
            }
        }
    }
    pub fn get_current_folder(&mut self) -> Option<&mut Folder> {
        self.root_dir.find_folder_mut(&self.current_path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileSystemItem {
    File_(File),
    Folder_(Folder),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub name: String,
    pub path: String,
    pub selected: bool,
    pub contents: Vec<FileSystemItem>,
    pub extension: String,
    pub file_filter_rules: Vec<FileFolderFilter>,
    pub folder_filter_rules: Vec<FolderFilter>,
    pub file_priority_rules: Vec<FileFolderPriority>,
    pub folder_priority_rules: Vec<FolderPriority>,
}

impl Folder {
    pub fn new(
        name: String,
        path: String,
        selected: bool,
        contents: Vec<FileSystemItem>,
        extension: String,
        file_filter_rules: Vec<FileFolderFilter>,
        folder_filter_rules: Vec<FolderFilter>,
        file_priority_rules: Vec<FileFolderPriority>,
        folder_priority_rules: Vec<FolderPriority>
    ) -> Self {
        Folder {
            name,
            path,
            selected,
            contents,
            extension,
            folder_filter_rules,
            file_filter_rules,
            file_priority_rules,
            folder_priority_rules,
        }
    }

    pub fn set_up_priority_by_file_folder(&mut self, priority: FileFolderPriority) {
        if let Ok(deep) = priority.deep.parse::<i32>() {
            if deep > -1 {
                if self.add_children_to_folder().is_ok() {
                    self.file_priority_rules.push(priority.clone());

                    self.contents
                        .iter_mut()
                        .filter_map(|item| {
                            match item {
                                FileSystemItem::Folder_(folder) => {
                                    Some(
                                        folder.set_up_priority_by_file_folder(
                                            FileFolderPriority::new(
                                                priority.regex.to_owned(),
                                                (deep - 1).to_string(),
                                                priority.content.to_owned(),
                                                priority.priority.to_owned()
                                            )
                                        )
                                    )
                                }
                                _ => { None }
                            }
                        })
                        .for_each(|_| {})
                }
            }
        }
    }
    pub fn edit_priority_by_file_folder(
        &mut self,
        new_priority: FileFolderPriority,
        old_priority: FileFolderPriority
    ) {
        let deep = new_priority.deep.clone().parse::<i32>();
        let old_deep = old_priority.deep.clone().parse::<i32>();

        if let (Ok(deep), Ok(old_deep)) = (deep, old_deep) {
            if old_deep <= -1 && deep <= -1 {
                return;
            }

            if let Ok(_) = self.add_children_to_folder() {
                let index = self.file_priority_rules
                    .iter()
                    .position(|folder| {
                        folder.deep == old_priority.deep &&
                            folder.regex == old_priority.regex &&
                            folder.priority == old_priority.priority &&
                            folder.content == old_priority.content
                    });

                if deep <= -1 {
                    if let Some(index) = index {
                        self.file_priority_rules.remove(index);
                    }
                } else {
                    if let Some(index) = index {
                        self.file_priority_rules[index] = new_priority.clone();
                    } else {
                        self.file_priority_rules.push(new_priority.clone());
                    }
                }

                for item in &mut self.contents {
                    if let FileSystemItem::Folder_(folder) = item {
                        folder.edit_priority_by_file_folder(
                            FileFolderPriority::new(
                                new_priority.regex.to_owned(),
                                (deep - 1).to_string(),
                                new_priority.content.to_owned(),
                                new_priority.priority.to_owned()
                            ),
                            FileFolderPriority::new(
                                old_priority.regex.to_owned(),
                                (old_deep - 1).to_string(),
                                old_priority.content.to_owned(),
                                old_priority.priority.to_owned()
                            )
                        );
                    }
                }
            }
        }
    }
    pub fn delete_priority_by_file_folder(&mut self, old_priority: FileFolderPriority) {
        if let Ok(old_deep) = old_priority.deep.parse::<i32>() {
            if old_deep > -1 {
                if
                    let Some(index) = self.file_priority_rules
                        .iter()
                        .position(
                            |folder|
                                folder.deep == old_priority.deep &&
                                folder.regex == old_priority.regex &&
                                folder.priority == old_priority.priority &&
                                folder.content == old_priority.content
                        )
                {
                    self.file_priority_rules.remove(index);
                }

                self.contents
                    .iter_mut()
                    .filter_map(|item| {
                        if let FileSystemItem::Folder_(folder) = item {
                            Some(
                                folder.delete_priority_by_file_folder(
                                    FileFolderPriority::new(
                                        old_priority.regex.to_owned(),
                                        (old_deep - 1).to_string(),
                                        old_priority.content.to_owned(),
                                        old_priority.priority.to_owned()
                                    )
                                )
                            )
                        } else {
                            None
                        }
                    })
                    .for_each(|_| {});
            }
        }
    }

    pub fn set_up_priority_by_folder(&mut self, priority: FolderPriority) {
        if let Ok(deep) = priority.deep.parse::<i32>() {
            if deep > -1 {
                if self.add_children_to_folder().is_ok() {
                    self.folder_priority_rules.push(priority.clone());

                    self.contents
                        .iter_mut()
                        .filter_map(|item| {
                            match item {
                                FileSystemItem::Folder_(folder) => {
                                    Some(
                                        folder.set_up_priority_by_folder(
                                            FolderPriority::new(
                                                priority.regex.to_owned(),
                                                (deep - 1).to_string(),
                                                priority.priority.to_owned()
                                            )
                                        )
                                    )
                                }
                                _ => { None }
                            }
                        })
                        .for_each(|_| {})
                }
            }
        }
    }
    pub fn edit_priority_by_folder(
        &mut self,
        new_priority: FolderPriority,
        old_priority: FolderPriority
    ) {
        let deep = new_priority.deep.clone().parse::<i32>();
        let old_deep = old_priority.deep.clone().parse::<i32>();

        if let (Ok(deep), Ok(old_deep)) = (deep, old_deep) {
            if old_deep <= -1 && deep <= -1 {
                return;
            }

            if let Ok(_) = self.add_children_to_folder() {
                let index = self.folder_priority_rules
                    .iter()
                    .position(
                        |folder|
                            folder.deep == old_priority.deep &&
                            folder.regex == old_priority.regex &&
                            folder.priority == old_priority.priority
                    );

                if deep <= -1 {
                    if let Some(index) = index {
                        self.folder_priority_rules.remove(index);
                    }
                } else {
                    if let Some(index) = index {
                        self.folder_priority_rules[index] = new_priority.clone();
                    } else {
                        self.folder_priority_rules.push(new_priority.clone());
                    }
                }

                for item in &mut self.contents {
                    if let FileSystemItem::Folder_(folder) = item {
                        folder.edit_priority_by_folder(
                            FolderPriority::new(
                                new_priority.regex.to_owned(),
                                (deep - 1).to_string(),
                                new_priority.priority.to_owned()
                            ),
                            FolderPriority::new(
                                old_priority.regex.to_owned(),
                                (old_deep - 1).to_string(),
                                old_priority.priority.to_owned()
                            )
                        );
                    }
                }
            }
        }
    }
    pub fn delete_priority_by_folder(&mut self, old_priority: FolderPriority) {
        if let Ok(old_deep) = old_priority.deep.parse::<i32>() {
            if old_deep > -1 {
                if
                    let Some(index) = self.folder_priority_rules
                        .iter()
                        .position(
                            |folder|
                                folder.deep == old_priority.deep &&
                                folder.regex == old_priority.regex &&
                                folder.priority == old_priority.priority
                        )
                {
                    self.folder_priority_rules.remove(index);
                }

                self.contents
                    .iter_mut()
                    .filter_map(|item| {
                        if let FileSystemItem::Folder_(folder) = item {
                            Some(
                                folder.delete_priority_by_folder(
                                    FolderPriority::new(
                                        old_priority.regex.to_owned(),
                                        (old_deep - 1).to_string(),
                                        old_priority.priority.to_owned()
                                    )
                                )
                            )
                        } else {
                            None
                        }
                    })
                    .for_each(|_| {});
            }
        }
    }

    pub fn set_up_filter_by_file_folder(&mut self, filter: FileFolderFilter) {
        if let Ok(deep) = filter.deep.parse::<i32>() {
            if deep > -1 {
                if self.add_children_to_folder().is_ok() {
                    self.file_filter_rules.push(filter.clone());

                    self.contents
                        .iter_mut()
                        .filter_map(|item| {
                            match item {
                                FileSystemItem::Folder_(folder) => {
                                    Some(
                                        folder.set_up_filter_by_file_folder(
                                            FileFolderFilter::new(
                                                filter.regex.to_owned(),
                                                filter.content.to_owned(),
                                                (deep - 1).to_string()
                                            )
                                        )
                                    )
                                }
                                _ => { None }
                            }
                        })
                        .for_each(|_| {})
                }
            }
        }
    }
    pub fn edit_filter_by_file_folder(
        &mut self,
        new_filter: FileFolderFilter,
        old_filter: FileFolderFilter
    ) {
        let deep = new_filter.deep.clone().parse::<i32>();
        let old_deep = old_filter.deep.clone().parse::<i32>();

        if let (Ok(deep), Ok(old_deep)) = (deep, old_deep) {
            if old_deep <= -1 && deep <= -1 {
                return;
            }

            if let Ok(_) = self.add_children_to_folder() {
                let index = self.file_filter_rules
                    .iter()
                    .position(
                        |folder|
                            folder.deep == old_filter.deep &&
                            folder.regex == old_filter.regex &&
                            folder.content == old_filter.content
                    );

                if deep <= -1 {
                    if let Some(index) = index {
                        self.file_filter_rules.remove(index);
                    }
                } else {
                    if let Some(index) = index {
                        self.file_filter_rules[index] = new_filter.clone();
                    } else {
                        self.file_filter_rules.push(new_filter.clone());
                    }
                }

                for item in &mut self.contents {
                    if let FileSystemItem::Folder_(folder) = item {
                        folder.edit_filter_by_file_folder(
                            FileFolderFilter::new(
                                new_filter.regex.to_owned(),
                                new_filter.content.to_owned(),
                                (deep - 1).to_string()
                            ),
                            FileFolderFilter::new(
                                old_filter.regex.to_owned(),
                                old_filter.content.to_owned(),
                                (old_deep - 1).to_string()
                            )
                        );
                    }
                }
            }
        }
    }
    pub fn delete_filter_by_file_folder(&mut self, old_filter: FileFolderFilter) {
        if let Ok(old_deep) = old_filter.deep.parse::<i32>() {
            if old_deep > -1 {
                if
                    let Some(index) = self.file_filter_rules
                        .iter()
                        .position(
                            |folder|
                                folder.deep == old_filter.deep &&
                                folder.regex == old_filter.regex &&
                                folder.content == old_filter.content
                        )
                {
                    self.file_filter_rules.remove(index);
                }

                self.contents
                    .iter_mut()
                    .filter_map(|item| {
                        if let FileSystemItem::Folder_(folder) = item {
                            Some(
                                folder.delete_filter_by_file_folder(
                                    FileFolderFilter::new(
                                        old_filter.regex.to_owned(),
                                        old_filter.content.to_owned(),
                                        (old_deep - 1).to_string()
                                    )
                                )
                            )
                        } else {
                            None
                        }
                    })
                    .for_each(|_| {});
            }
        }
    }
    pub fn filter_by_file_folder(&mut self) {
        let filters = &self.file_filter_rules.to_owned();

        if filters.is_empty() {
            return;
        }

        let new_content: Vec<FileSystemItem> = self.contents
            .iter()
            .filter(|&item| {
                filters.iter().any(|filter| {
                    if let FileSystemItem::File_(file) = item {
                        let regex = Regex::new(filter.regex.as_str()).unwrap();
                        if regex.is_match(&file.extension) {
                            if filter.content.is_empty() {
                                return true;
                            }

                            if let Ok(file_content) = file_service::get_file_content(&file.path) {
                                let regex = Regex::new(filter.content.as_str()).unwrap();
                                return regex.is_match(&file_content);
                            }
                        }
                        false
                    } else {
                        true
                    }
                })
            })
            .cloned()
            .collect();

        self.contents = new_content;
    }

    pub fn set_up_filter_by_folder(&mut self, filter: FolderFilter) {
        if let Ok(deep) = filter.deep.parse::<i32>() {
            if deep > -1 {
                if self.add_children_to_folder().is_ok() {
                    self.folder_filter_rules.push(filter.clone());

                    self.contents
                        .iter_mut()
                        .filter_map(|item| {
                            if let FileSystemItem::Folder_(folder) = item {
                                Some(
                                    folder.set_up_filter_by_folder(
                                        FolderFilter::new(
                                            filter.regex.to_owned(),
                                            (deep - 1).to_string()
                                        )
                                    )
                                )
                            } else {
                                None
                            }
                        })
                        .for_each(|_| {});
                }
            }
        }
    }
    pub fn delete_filter_by_folder(&mut self, old_filter: FolderFilter) {
        if let Ok(old_deep) = old_filter.deep.parse::<i32>() {
            if old_deep > -1 {
                if
                    let Some(index) = self.folder_filter_rules
                        .iter()
                        .position(
                            |folder|
                                folder.deep == old_filter.deep && folder.regex == old_filter.regex
                        )
                {
                    self.folder_filter_rules.remove(index);
                }

                self.contents
                    .iter_mut()
                    .filter_map(|item| {
                        if let FileSystemItem::Folder_(folder) = item {
                            Some(
                                folder.delete_filter_by_folder(
                                    FolderFilter::new(
                                        old_filter.regex.to_owned(),
                                        (old_deep - 1).to_string()
                                    )
                                )
                            )
                        } else {
                            None
                        }
                    })
                    .for_each(|_| {});
            }
        }
    }
    pub fn edit_filter_by_folder(&mut self, new_filter: FolderFilter, old_filter: FolderFilter) {
        let deep = new_filter.deep.clone().parse::<i32>();
        let old_deep = old_filter.deep.clone().parse::<i32>();

        if let (Ok(deep), Ok(old_deep)) = (deep, old_deep) {
            if old_deep <= -1 && deep <= -1 {
                return;
            }

            if let Ok(_) = self.add_children_to_folder() {
                let index = self.folder_filter_rules
                    .iter()
                    .position(
                        |folder| folder.deep == old_filter.deep && folder.regex == old_filter.regex
                    );

                if deep <= -1 {
                    if let Some(index) = index {
                        self.folder_filter_rules.remove(index);
                    }
                } else {
                    if let Some(index) = index {
                        self.folder_filter_rules[index] = new_filter.clone();
                    } else {
                        self.folder_filter_rules.push(new_filter.clone());
                    }
                }

                for item in &mut self.contents {
                    if let FileSystemItem::Folder_(folder) = item {
                        folder.edit_filter_by_folder(
                            FolderFilter::new(new_filter.regex.to_owned(), (deep - 1).to_string()),
                            FolderFilter::new(
                                old_filter.regex.to_owned(),
                                (old_deep - 1).to_string()
                            )
                        );
                    }
                }
            }
        }
    }
    pub fn filter_by_folder(&mut self) {
        let filters = &self.folder_filter_rules.to_owned();

        if filters.is_empty() {
            return;
        }

        let new_content: Vec<FileSystemItem> = self.contents
            .iter()
            .filter(|&item| {
                filters.iter().any(|regex| {
                    if let FileSystemItem::Folder_(folder) = item {
                        let regex = Regex::new(regex.regex.as_str()).unwrap();
                        regex.is_match(&folder.name)
                    } else {
                        true
                    }
                })
            })
            .cloned()
            .collect();

        self.contents = new_content;
    }

    pub fn add_children_to_folder(&mut self) -> Result<(), std::io::Error> {
        let content = file_service::get_system_items_from_path(&self.path)?;

        self.add_existing_items(content.clone());

        self.delete_not_existing_items(content);

        self.filter_by_folder();

        self.filter_by_file_folder();

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

        let result: Option<&mut Folder> = self.contents.par_iter_mut().find_map_any(|content| {
            if let FileSystemItem::Folder_(folder) = content {
                if &folder.path == path {
                    Some(folder)
                } else if path.contains(&folder.path) {
                    folder.find_folder_mut(path)
                } else {
                    None
                }
            } else {
                None
            }
        });

        result
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
    pub fn add_existing_items(&mut self, items: Vec<FileSystemItem>) {
        items.iter().for_each(|item| self.add_existing_item(item.clone()))
    }
    pub fn add_existing_item(&mut self, item: FileSystemItem) {
        if
            !self.contents.iter().any(|existing_item| {
                match (existing_item, &item) {
                    (FileSystemItem::File_(existing_file), FileSystemItem::File_(new_file)) =>
                        existing_file.name == new_file.name,
                    (
                        FileSystemItem::Folder_(existing_folder),
                        FileSystemItem::Folder_(new_folder),
                    ) => existing_folder.name == new_folder.name,
                    _ => false,
                }
            })
        {
            self.contents.push(item);
        }
    }
    pub fn delete_not_existing_items(&mut self, items: Vec<FileSystemItem>) {
        let unique_paths: HashSet<String> = items
            .iter()
            .map(|item| {
                match item {
                    FileSystemItem::File_(file) => { file.path.to_owned() }
                    FileSystemItem::Folder_(folder) => { folder.path.to_owned() }
                }
            })
            .collect();

        let mut remove_indexes = Vec::new();

        for (index, content_item) in self.contents.iter_mut().enumerate() {
            match content_item {
                FileSystemItem::File_(file) => {
                    if !unique_paths.contains(&file.path) {
                        remove_indexes.push(index);
                    }
                }
                FileSystemItem::Folder_(folder) => {
                    if !unique_paths.contains(&folder.path) {
                        remove_indexes.push(index);
                    }
                }
            }
        }

        for (index, current_index) in remove_indexes.iter().enumerate() {
            self.contents.remove(current_index - index);
        }
    }
    pub fn select_deep_all(&mut self, bool: bool) {
        if let Err(_) = self.add_children_to_folder() {
            return;
        }

        self.selected = bool;

        self.contents.par_iter_mut().for_each(|item| {
            if let FileSystemItem::File_(file) = item {
                file.selected = bool;
            } else if let FileSystemItem::Folder_(folder) = item {
                folder.select_deep_all(bool);
            }
        });
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub path: String,
    pub selected: bool,
    pub extension: String,
    pub file_priority_rules: Vec<FilePriority>,
}

impl File {
    pub fn new(
        name: String,
        path: String,
        selected: bool,
        extension: String,
        file_priority_rules: Vec<FilePriority>
    ) -> Self {
        File {
            name,
            path,
            selected,
            extension,
            file_priority_rules,
        }
    }
}
