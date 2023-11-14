use std::path::Path;
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use crate::app::{App, AppMode};
use crate::file_service;
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
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.table.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table.select(Some(i));
    }

    pub fn open(&mut self) -> Result<(), std::io::Error>{
        let current_dir =
            self.root.find_folder_by_path(&self.root.current_path.clone());
        if let Some(dir) = current_dir {
            let index = self
                .table.selected();


            if let Some(index) = index {
                let new_dir = dir.contents.get_mut(index);

                if let Some(new_dir) = new_dir {
                    match new_dir {
                        FileSystemItem::File_(_) => {}
                        FileSystemItem::Folder_(fodlder) => {
                            fodlder.contents = file_service::get_system_items_from_path(fodlder.path.clone())?;
                            fodlder.sort_contents();
                            let content_len = fodlder.contents.len();

                            self.root.current_path = fodlder.path.clone();

                            if content_len > 0 {
                                self.set_index_table(Some(0));
                            } else {
                                self.set_index_table(None);
                            }

                            self.root.history_index.push(index);
                        }
                    }
                }
            }
        }
        Ok(())
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
        let index =  self.root.history_index.pop();

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


    pub fn init_index_table(&mut self) {
        let selected = self.table.selected();
        let length = self.root.rows.len();

        if length > 0 {
            if let None = selected {
                self.set_index_table(Some(0));
            }
        }
    }

    pub fn set_index_table(&mut self, index: Option<usize>)  {
        self.table.select(index);
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
                app.file_list.open()?;
            }
            KeyCode::Left => {
                app.file_list.close();
            },
            KeyCode::Char(' ') => {
                app.file_list.select();
            }
            _ => {}
        }

        Ok(())
    }

    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow);
        let normal_style = Style::default().bg(Color::White);
        let header_cells = ["", "Name", "Extension", "Access", "Size"]
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
            .block(Block::default().borders(Borders::ALL).title_alignment(Alignment::Center)
                .title(app.file_list.root.current_path.as_str()))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Length(3),
                Constraint::Length(40),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10)
            ]);
        f.render_stateful_widget(t, chunks[1], &mut app.file_list.table);
    }
}