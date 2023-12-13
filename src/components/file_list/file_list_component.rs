use std::path::PathBuf;

use crate::{ utils::table_util, application::{ app::App, app_mode::AppMode } };

use super::file_list_state::FileListState;

const HELP: &'static str =
    "| ESC~Back | ↑ ↓ ← → Move | SPACE~Select | s~Select Deep | a~Select All | f~filter | p~priority |";

pub struct FileListComponent {
    pub state: FileListState,
}

impl FileListComponent {
    pub fn init() -> Result<Self, std::io::Error> {
        Ok(FileListComponent { state: FileListState::init()? })
    }

    pub fn move_up(&mut self) {
        table_util::move_up(&mut self.state.table_state, self.state.table_rows.len());
    }

    pub fn move_down(&mut self) {
        table_util::move_down(&mut self.state.table_state, self.state.table_rows.len());
    }

    pub fn open(&mut self) -> Result<(), std::io::Error> {
        if let Some(entry) = self.state.get_selected_entry() {
            if !entry.is_dir() {
                return Ok(());
            }

            let path = entry.path.clone();

            entry.renew_children()?;

            self.state.history.push(self.state.table_state.selected().unwrap());
            self.state.table_state.select(Some(0));

            self.state.current_path = path;
            self.state.set_rows();
        }

        Ok(())
    }

    pub fn close(&mut self) {
        let mut new_path = self.state.current_path.clone();
        new_path.pop();

        match new_path == self.state.current_path {
            true => {
                if new_path.to_str().map_or(false, |s| s.contains(":")) {
                    self.state.current_path = PathBuf::from("/");
                } else {
                    return;
                }
            }
            false => {
                self.state.current_path = new_path;
            }
        }

        self.state.set_rows();
        self.state.table_state.select(self.state.history.pop());
    }

    pub fn select(&mut self) -> Result<(), std::io::Error> {
        match self.state.select() {
            Ok(_) => self.state.set_rows(),
            _ => {}
        }

        Ok(())
    }

    pub fn select_deep(&mut self) -> Result<(), std::io::Error> {
        match self.state.select_deep() {
            Ok(_) => self.state.set_rows(),
            _ => {}
        }

        Ok(())
    }

    pub fn select_all(&mut self) {
        self.state.select_all();
        self.state.set_rows();
    }

    pub fn open_filter(app: &mut App) {
        let file_list = &mut app.components.file_list;
        if file_list.state.is_selected_dir() {
            file_list.state.is_priority_mode = false;
            app.components.file_filter.state.init_index_table();
            app.change_mode(AppMode::FileFilter, AppMode::FileList);
        }
    }
    pub fn open_priority(app: &mut App) {
        let file_list = &mut app.components.file_list;

        if file_list.state.is_selected() {
            file_list.state.is_priority_mode = true;

            match file_list.state.is_selected_dir() {
                true => {
                    app.components.dir_file_priority.state.init_index_table();
                    app.change_mode(AppMode::DirFilePriority, AppMode::FileList)
                }
                false => app.change_mode(AppMode::FilePriority, AppMode::FileList),
            }
        }
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
