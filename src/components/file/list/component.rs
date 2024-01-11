use std::path::PathBuf;

use crate::{
    application::{
        app::App,
        mode::{AppMode, TemplateForm},
    },
    services::{entry_filter_service, entry_priority_service},
    utils::table_util,
};

use super::state::FileListState;

const HELP: &'static str =
    "| ESC~Back | ↑ ↓ ← → Move | SPACE~Select | s~Select Deep | a~Select All | f~Edit Settings | p~Settings | c~Create/Edit | n~New |";

pub struct FileListComponent {
    pub state: FileListState,
}

impl FileListComponent {
    pub fn init() -> Result<Self, std::io::Error> {
        Ok(FileListComponent {
            state: FileListState::init()?,
        })
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        self.state = FileListState::init()?;
        self.state.table_state.select(Some(0));
        Ok(())
    }

    pub fn move_up(&mut self) {
        table_util::move_up(&mut self.state.table_state, self.state.table_rows.len());
    }

    pub fn move_down(&mut self) {
        table_util::move_down(&mut self.state.table_state, self.state.table_rows.len());
    }

    pub fn save(app: &mut App) {
        app.change_mode(
            AppMode::TemplateForm(TemplateForm::Name),
            app.state.mode.clone(),
        );
    }

    pub fn open(&mut self) -> Result<(), std::io::Error> {
        if let Some(entry) = self.state.get_selected_entry() {
            if !entry.is_dir() {
                return Ok(());
            }

            let path = entry.path.clone();

            entry.renew_children()?;

            entry_filter_service::filter(entry);
            entry_priority_service::priority(entry);

            self.state
                .history
                .push(self.state.table_state.selected().unwrap());
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

        let entry = self.state.get_selected_entry().unwrap();

        entry_filter_service::delete_not_root(entry);
        entry_priority_service::delete_not_root(entry);
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

    pub fn open_edit_settings(app: &mut App) {
        app.components.file_list_settings.state.init_index_table();
        app.change_mode(AppMode::FileListSettings, AppMode::FileList);
    }

    pub fn open_settings(app: &mut App) {
        let file_list = &mut app.components.file_list;
        let settings = &mut app.components.file_list_settings;

        if settings.state.seleted_items.is_empty() {
            return;
        }

        if file_list.state.is_selected() {
            match file_list.state.is_selected_dir() {
                true => {
                    let mut settings = settings.state.seleted_items.clone();
                    settings.sort();

                    match settings[0] {
                        0 => app.change_mode(AppMode::FileFilter, AppMode::FileList),
                        1 => app.change_mode(AppMode::DirFilePriority, AppMode::FileList),
                        2 => app.change_mode(AppMode::DirPriority, AppMode::FileList),
                        _ => {}
                    }
                }
                false => {
                    if settings.state.seleted_items.contains(&3) {
                        app.change_mode(AppMode::FilePriority, AppMode::FileList)
                    }
                }
            }
        }
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
