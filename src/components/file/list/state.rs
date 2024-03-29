use std::path::PathBuf;

use tui::{style::Color, widgets::TableState};

use crate::{
    application::mode::AppMode,
    models::dir_entry::DirEntry,
    services::{file_service, file_system_service},
    utils::table_util,
};

#[derive(Default)]
pub struct FileListState {
    pub table_state: TableState,
    pub root: DirEntry,
    pub table_rows: Vec<(Vec<String>, Color)>,
    pub current_path: PathBuf,
    pub history: Vec<usize>,
    pub is_priority_mode: bool,
    pub settings: Vec<AppMode>,
}

pub struct Settings {
    pub is_file_priority: bool,
    pub is_dir_priority: bool,
    pub is_dir_file_priority: bool,
    pub id_file_filter: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            is_file_priority: true,
            is_dir_priority: true,
            is_dir_file_priority: true,
            id_file_filter: true,
        }
    }
}

impl FileListState {
    pub fn init() -> Result<Self, std::io::Error> {
        let mut state = FileListState::default();

        state.root = DirEntry::default();
        state.root.path = PathBuf::from("/");
        state.root.children = Some(file_service::root()?);
        state.root.sort_children();
        state.current_path = PathBuf::from("/");
        state.set_rows();

        Ok(state)
    }

    pub fn is_selected_dir(&mut self) -> bool {
        if let Some(entry) = self.get_selected_entry() {
            return entry.is_dir();
        }

        false
    }

    pub fn init_index_table(&mut self) {
        let len = self.table_rows.len();
        table_util::init_index_table(&mut self.table_state, len)
    }
    pub fn is_selected(&self) -> bool {
        table_util::is_selected(&self.table_state)
    }
    pub fn get_selected_entry(&mut self) -> Option<&mut DirEntry> {
        if let Some(index) = self.table_state.selected() {
            if let Some(entry) =
                file_system_service::find_in_dir(&mut self.root, self.current_path.as_path())
            {
                if !entry.is_dir() {
                    return None;
                }

                if let Some(children) = entry.children.as_mut() {
                    return children.get_mut(index);
                }
            }
        }

        None
    }
    pub fn select(&mut self) -> Result<(), std::io::Error> {
        if let Some(entry) = self.get_selected_entry() {
            entry.set_select(!entry.selected);
        }
        Ok(())
    }
    pub fn select_deep(&mut self) -> Result<(), std::io::Error> {
        if let Some(entry) = self.get_selected_entry() {
            match entry.is_dir() {
                true => entry.select_deep_entries(!entry.selected),
                false => entry.set_select(!entry.selected),
            };
        }
        Ok(())
    }
    pub fn select_all(&mut self) {
        if let Some(entry) =
            file_system_service::find_in_dir(&mut self.root, self.current_path.as_path())
        {
            if entry.is_dir() {
                entry.select_all();
            }
        }
    }
    pub fn rows(&mut self) -> Vec<(Vec<String>, Color)> {
        if let Some(entry) =
            file_system_service::find_in_dir(&mut self.root, self.current_path.as_path())
        {
            if !entry.is_dir() {
                return vec![];
            }

            match &entry.children {
                Some(children) => {
                    return children
                        .iter()
                        .map(|c| c.row())
                        .collect::<Vec<(Vec<String>, Color)>>();
                }
                None => {
                    return vec![];
                }
            }
        }

        vec![]
    }
    pub fn set_rows(&mut self) {
        self.table_rows = self.rows();

        if self.rows().len() == 0 {
            self.table_state.select(None);
        }
    }
}
