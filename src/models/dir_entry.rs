use std::path::PathBuf;
use rayon::prelude::*;

use tui::style::Color;

use crate::services::file_service;

const DIR_COLOR: Color = Color::Green;
const FILE_COLOR: Color = Color::Blue;

#[derive(Default, Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub children: Option<Vec<DirEntry>>,
    pub selected: bool,
}

impl DirEntry {
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn file_name(&self) -> String {
        let file_name = self.path.file_name().and_then(|name| name.to_str());

        match file_name {
            Some(name) => name.to_string(),
            None => self.path.to_string_lossy().to_string(),
        }
    }

    pub fn extension(&self) -> String {
        if self.is_dir() {
            return "dir".to_string();
        }

        let extension = self.path.extension().and_then(|ext| ext.to_str());

        match extension {
            Some(ext) => ext.to_string(),
            None => "file".to_string(),
        }
    }

    pub fn color(&self) -> Color {
        match self.is_dir() {
            true => DIR_COLOR,
            false => FILE_COLOR,
        }
    }

    pub fn row(&self) -> (Vec<String>, Color) {
        let brackets = match self.selected {
            true => "[x]".to_string(),
            false => "[ ]".to_string(),
        };

        (vec![brackets, self.file_name(), self.extension()], self.color())
    }

    pub fn renew_children(&mut self) -> Result<(), std::io::Error> {
        if self.is_dir() {
            let new_children = Some(file_service::entries(self.path.as_path())?);

            self.compare_old_and_new_children(new_children.unwrap_or_default());
            self.sort_children();
        }

        Ok(())
    }

    pub fn get_selected(&mut self) -> Vec<DirEntry> {
        let mut selected = vec![];

        if let Some(children) = self.children.as_mut() {
            selected = children.clone();
            selected.retain(|entry| entry.selected);
        }
        selected
    }

    pub fn sort_children(&mut self) {
        if let Some(children) = self.children.take() {
            let mut files: Vec<DirEntry> = Vec::new();
            let mut dirs: Vec<DirEntry> = Vec::new();

            for child in children.into_iter() {
                if child.path.is_dir() {
                    dirs.push(child);
                } else {
                    files.push(child);
                }
            }

            dirs.sort_by(|a, b| a.path.cmp(&b.path));
            files.sort_by(|a, b| a.path.cmp(&b.path));

            dirs.append(&mut files);

            self.children = Some(dirs);
        }
    }

    pub fn compare_old_and_new_children(&mut self, new_children: Vec<DirEntry>) {
        let mut selected = self.get_selected();

        for entry in new_children {
            if !selected.iter().any(|selected_entry| selected_entry.path == entry.path) {
                selected.push(entry);
            }
        }

        self.children = Some(selected);
    }

    pub fn set_select(&mut self, bool: bool) {
        self.selected = bool;
    }

    pub fn select_dir_entries(&mut self, bool: bool) -> Result<(), std::io::Error> {
        self.renew_children()?;

        self.selected = bool;

        if let Some(children) = self.children.as_mut() {
            for entry in children {
                entry.selected = bool;
            }
        }

        Ok(())
    }

    pub fn select_all(&mut self) {
        if let Some(children) = self.children.as_mut() {
            for entry in children {
                let _ = entry.select_dir_entries(!entry.selected);
            }
        }
    }

    pub fn select_deep_entries(&mut self, bool: bool) {
        if let Err(_) = self.renew_children() {
            return;
        }

        self.selected = bool;

        self.children
            .as_mut()
            .unwrap()
            .par_iter_mut()
            .for_each(|entry| {
                if entry.is_dir() {
                    entry.select_deep_entries(bool)
                } else {
                    entry.selected = bool;
                }
            });
    }
}
