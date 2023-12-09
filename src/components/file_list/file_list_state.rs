use std::path::PathBuf;

use tui::{ widgets::TableState, style::Color };

use crate::{
    utils::table_util,
    models::dir_entry::DirEntry,
    services::{ file_service, file_system_service },
};

#[derive(Default)]
pub struct FileListState {
    pub table_state: TableState,
    pub root: DirEntry,
    pub table_rows: Vec<(Vec<String>, Color)>,
    pub current_path: PathBuf,
    pub history: Vec<usize>,
}

impl FileListState {
    pub fn init() -> Result<Self, std::io::Error> {
        let mut state = FileListState::default();

        state.root = DirEntry::default();
        state.root.path = PathBuf::from("/");
        state.root.children = Some(file_service::root()?);
        state.current_path = PathBuf::from("/");
        state.set_rows();

        Ok(state)
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
            if
                let Some(entry) = file_system_service::find_in_dir(
                    &mut self.root,
                    self.current_path.as_path()
                )
            {
                if let Some(children) = entry.children.as_mut() {
                    return children.get_mut(index);
                }
            }
        }

        None
    }

    pub fn rows(&mut self) -> Vec<(Vec<String>, Color)> {
        if
            let Some(dir) = file_system_service::find_in_dir(
                &mut self.root,
                self.current_path.as_path()
            )
        {
            match &dir.children {
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
