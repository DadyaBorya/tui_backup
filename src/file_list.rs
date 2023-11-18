use std::io::{ErrorKind};
use std::path::Path;
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Rect, Layout, Direction};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, BorderType, Cell, Row, Table, TableState};
use crate::app::{App, AppMode};
use crate::file_list_filter::FileItemListFilter;
use crate::file_system::{FileSystem, FileSystemItem};

#[derive(Debug)]
pub struct FileList {
    pub root: FileSystem,
    pub table: TableState,
}

impl FileList {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(FileList {
            root: FileSystem::new()?,
            table: TableState::default(),
        })
    }
        pub fn next(&mut self) {
        let i = match self.table.selected() {
            Some(i) => {
                if i >= self.root.rows.len() - 1 {
                    Some(i)
                } else {
                    Some(i + 1)
                }
            }
            None => None,
        };
        self.table.select(i);
    }
    pub fn previous(&mut self) {
        let i = match self.table.selected() {
            Some(i) => {
                if i == 0 {
                    Some(i)
                } else {
                    Some(i - 1)
                }
            }
            None => None,
        };
        self.table.select(i);
    }

    pub fn open(&mut self) -> Result<(), std::io::Error> {
        if let Some(index) = self.table.selected() {
            let new_path = self.open_folder(index)?;
            self.root.current_path = new_path;
        }
        Ok(())
    }
    fn open_folder(&mut self, index: usize) -> Result<String, std::io::Error> {
        if let Some(current_folder) = self.root.get_current_folder() {
            if let Some(folder) = current_folder.find_folder_mut_in_content(index) {
                folder.add_children_to_folder()?;

                let current_path = folder.path.clone();

                if current_folder.contents.len() > 0 {
                    self.set_index_table(Some(0));
                } else {
                    self.set_index_table(None);
                }

                self.root.history_index.push(index);

                return Ok(current_path);
            }
        }
        Ok(self.root.current_path.clone())
    }
    pub fn close(&mut self) {
        if self.root.current_path == "/" {
            self.root.history_index.clear();
            return;
        }
        let paths: Vec<&str> = self.root.current_path.split('/').filter(|path| !path.is_empty()).collect();

        if paths.len() == 1 {
            self.root.current_path = "/".to_string();
        } else {
            if let Some(parent) = Path::new(&self.root.current_path).parent() {
                self.root.current_path = parent.to_str().unwrap().to_string();
            }
        }
        let index = self.root.history_index.pop();

        if index.is_none() {
            self.set_index_table(Some(0))
        } else {
            self.set_index_table(index);
        }
    }
    pub fn select(&mut self) {
        if let Some(index) = self.table.selected() {
            self.root.select(index);
        }
    }
    pub fn select_all(&mut self) {
        self.root.select_all();
    }
    pub fn init_index_table(&mut self) {
        let selected = self.table.selected();
        let length = self.root.rows.len();

        if length > 0 {
            if let None = selected {
                self.set_index_table(Some(0));
            }
        }
    }
    pub fn set_index_table(&mut self, index: Option<usize>) {
        self.table.select(index);
    }
    pub fn get_current_item(&mut self) -> Option<&mut FileSystemItem> {
        if let Some(folder) = self.root.root_dir.find_folder_mut(&self.root.current_path.clone()) {
            if let Some(index) = self.table.selected() {
                return Some(&mut folder.contents[index]);
            }
        }
        None
    }

    pub fn show_error(app: &mut App, error: std::io::Error) {
        if let ErrorKind::PermissionDenied = error.kind() {
            app.error = Some(error.to_string());
            app.change_mode(AppMode::ErrorPopup);
        }
    }
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Esc => {
                app.file_list.set_index_table(None);
                app.change_mode(AppMode::Tab);
            }
            KeyCode::Up => {
                app.file_list.previous();
            }
            KeyCode::Down => {
                app.file_list.next();
            }
            KeyCode::Right => {
                if let Err(error) = app.file_list.open() {
                    FileList::show_error(app, error);
                }
            }
            KeyCode::Left => {
                app.file_list.close();
            }
            KeyCode::Char(' ') => {
                app.file_list.select();
            }
            KeyCode::Char('a') => {
                app.file_list.select_all();
            }
            KeyCode::Char('f') => {
                if let Some(item) = app.file_list.get_current_item() {
                    if let FileSystemItem::Folder_(_) = item {
                        app.change_mode(AppMode::FolderListFilter);
                    }
                }
            }
            KeyCode::Char('F') => {
                if let Some(item) = app.file_list.get_current_item() {
                    if let FileSystemItem::Folder_(_) = item {
                        app.change_mode(AppMode::FileListFilter);
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let list_chunks = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50), Constraint::Percentage(50)
                ].as_ref()
            ).split(chunks[1]);

        let action_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50), Constraint::Percentage(50)
                ].as_ref()
            ).split(list_chunks[1]);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow);
        let normal_style = Style::default().bg(Color::White);
        let header_cells = ["", "Name", "Extension"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));

        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        app.file_list.root.set_rows_of_current_dir();


        let rows = app.file_list.root.rows.iter().map(|item| {
            let height = item.0
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.0.iter()
                .map(|c| Cell::from(c.as_str()));
            Row::new(cells).style(Style::default().fg(item.1)).height(height as u16).bottom_margin(1)
        });

        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title_alignment(Alignment::Center)
                .title(app.file_list.root.current_path.as_str()))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Length(3),
                Constraint::Length(40),
                Constraint::Min(10),
            ]);
        f.render_stateful_widget(t, list_chunks[0], &mut app.file_list.table);

        FileItemListFilter::ui(app, f, &action_chunks);

        let priority =
            Block::default().title("Priority").borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title_alignment(Alignment::Center);

        f.render_widget(priority, action_chunks[1]);
    }
}