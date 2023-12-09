use std::path::PathBuf;

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
        let extension = self.path.extension().and_then(|ext| ext.to_str());

        match extension {
            Some(ext) => ext.to_string(),
            None => "dir".to_string(),
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
            false => "[]".to_string(),
        };

        (vec![brackets, self.file_name(), self.extension()], self.color())
    }

    pub fn renew_children(&mut self) -> Result<(), std::io::Error> {
        if self.is_dir() {
            self.children = Some(file_service::entries(self.path.as_path())?);
        }

        Ok(())
    }
}
