use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Rect, Layout, Direction, Constraint, Alignment};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, BorderType, List, ListItem, ListState};
use crate::app::{App};
use crate::app_mode::{AppMode, FileFolderListFilter, FolderListFilter, FolderListPriority};
use crate::file_system::{FileSystemItem};

#[derive(Debug, Clone)]
pub struct FileFolderFilter {
    pub regex: String,
    pub content: String,
    pub deep: String,
}

impl FileFolderFilter {
    pub fn new(regex: String, content: String, deep: String) -> Self {
        FileFolderFilter {
            regex,
            content,
            deep,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FolderFilter {
    pub regex: String,
    pub deep: String,
}

impl FolderFilter {
    pub fn new(regex: String, deep: String) -> Self {
        FolderFilter {
            regex,
            deep,
        }
    }
}

pub struct FileItemListFilter {
    pub file_filter_rules: Vec<FileFolderFilter>,
    pub file_filter_list: ListState,
    pub folder_filter_rules: Vec<FolderFilter>,
    pub folder_filter_list: ListState,
    pub new_regex: String,
    pub new_deep: String,
    pub new_content: String
}


impl FileItemListFilter {
    pub fn new() -> Self {
        FileItemListFilter {
            folder_filter_rules: vec![],
            file_filter_rules: vec![],
            file_filter_list: ListState::default(),
            folder_filter_list: ListState::default(),
            new_regex: "".to_string(),
            new_deep: "".to_string(),
            new_content: "".to_string()
        }
    }
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let list_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50), Constraint::Percentage(50)
                ].as_ref()
            ).split(chunks[0]);

        if let Some(item) = app.file_list.get_current_item() {
            if let FileSystemItem::Folder_(folder) = item {
                app.file_item_list_filter.folder_filter_rules = folder.folder_filter_rules.to_owned();
                app.file_item_list_filter.file_filter_rules = folder.file_filter_rules.to_owned();

                let folder_items: Vec<ListItem> = folder.folder_filter_rules.to_owned().into_iter()
                    .map(|item| {
                        ListItem::new(format!("{} ({})", item.regex, item.deep))
                    }).collect();

                let folder_list = List::new(folder_items)
                    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Folder Filter").title_alignment(Alignment::Center))
                    .style(match app.mode {
                        AppMode::FolderListFilter(FolderListFilter::List) => Style::default().fg(Color::Yellow),
                        _ => Style::default()
                    })
                    .highlight_symbol("->")
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                f.render_stateful_widget(folder_list, list_chunk[0], &mut app.file_item_list_filter.folder_filter_list);

                let file_items: Vec<ListItem> = folder.file_filter_rules.to_owned().into_iter()
                    .map(|item| {
                        ListItem::new(format!("{} ({})\n{}", item.regex, item.deep, item.content))
                    }).collect();

                let file_list = List::new(file_items)
                    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("File Filter").title_alignment(Alignment::Center))
                    .style(match app.mode {
                        AppMode::FileFolderListFilter(FileFolderListFilter::List) => Style::default().fg(Color::Yellow),
                        _ => Style::default(),
                    })
                    .highlight_symbol("->")
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                f.render_stateful_widget(file_list, list_chunk[1], &mut app.file_item_list_filter.file_filter_list);
            }
        }
    }
    pub fn clean_inputs(&mut self) {
        self.new_content.clear();
        self.new_deep.clear();
        self.new_regex.clear();
    }
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FolderListFilter(FolderListFilter::List) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileList);
                        app.file_item_list_filter.folder_filter_list.select(None);
                    }
                    KeyCode::Char('n') => {
                        app.change_mode(AppMode::FolderListFilter(FolderListFilter::Form));
                    }
                    KeyCode::Up => {
                        if app.file_item_list_filter.folder_filter_rules.is_empty() {
                            app.file_item_list_filter.folder_filter_list.select(None);
                        } else {
                            let selected = match app.file_item_list_filter.folder_filter_list.selected() {
                                Some(v) => {
                                    if v == 0 {
                                        Some(v)
                                    } else {
                                        Some(v - 1)
                                    }
                                }
                                None => {
                                    Some(0)
                                }
                            };
                            app.file_item_list_filter.folder_filter_list.select(selected);
                        }
                    }
                    KeyCode::Down => {

                        if app.file_item_list_filter.folder_filter_rules.is_empty() {
                            app.file_item_list_filter.folder_filter_list.select(None);
                        } else {
                            let selected = match app.file_item_list_filter.folder_filter_list.selected() {
                                Some(v) => {
                                    if v == app.file_item_list_filter.folder_filter_rules.len() - 1 {
                                        Some(v)
                                    } else {
                                        Some(v + 1)
                                    }
                                }
                                None => {
                                    Some(0)
                                }
                            };
                            app.file_item_list_filter.folder_filter_list.select(selected);
                        }
                    }
                    KeyCode::Char('d') => {
                        if let Some(index) = app.file_item_list_filter.folder_filter_list.selected() {
                            if let Some(item) = app.file_list.get_current_item() {
                                match item {
                                    FileSystemItem::File_(_) => {}
                                    FileSystemItem::Folder_(folder) => {
                                        if folder.folder_filter_rules.len() > 0 && index < folder.folder_filter_rules.len(){
                                            folder.folder_filter_rules.remove(index);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('e') => {
                        if let Some(index) = app.file_item_list_filter.folder_filter_list.selected() {
                            if let Some(item) = app.file_list.get_current_item() {
                                match item {
                                    FileSystemItem::File_(_) => {}
                                    FileSystemItem::Folder_(folder) => {
                                        let rule = &folder.folder_filter_rules[index];
                                        app.file_item_list_filter.new_regex = rule.regex.to_owned();
                                        app.file_item_list_filter.new_deep = rule.deep.to_owned();
                                        app.is_edit_folder_filter_form_popup = true;
                                        app.change_mode(AppMode::FolderListFilter(FolderListFilter::Form));
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::List)),
                    _ => {}
                }
            }
            AppMode::FileFolderListFilter(FileFolderListFilter::List) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileList);
                        app.file_item_list_filter.file_filter_list.select(None);
                    }
                    KeyCode::Char('n') => {
                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Form))
                    },
                    KeyCode::Up => {
                        if app.file_item_list_filter.file_filter_rules.is_empty() {
                            app.file_item_list_filter.file_filter_list.select(None);
                        } else {
                            let selected = match app.file_item_list_filter.file_filter_list.selected() {
                                Some(v) => {
                                    if v == 0 {
                                        Some(v)
                                    } else {
                                        Some(v - 1)
                                    }
                                }
                                None => {
                                    Some(0)
                                }
                            };
                            app.file_item_list_filter.file_filter_list.select(selected);
                        }
                    }
                    KeyCode::Down => {
                        if app.file_item_list_filter.file_filter_rules.is_empty() {
                            app.file_item_list_filter.file_filter_list.select(None);
                        } else {
                            let selected = match app.file_item_list_filter.file_filter_list.selected() {
                                Some(v) => {
                                    if v == app.file_item_list_filter.file_filter_rules.len() - 1 {
                                        Some(v)
                                    } else {
                                        Some(v + 1)
                                    }
                                }
                                None => {
                                    Some(0)
                                }
                            };
                            app.file_item_list_filter.file_filter_list.select(selected);
                        }
                    },
                    KeyCode::Char('d') => {
                        if let Some(index) = app.file_item_list_filter.file_filter_list.selected() {
                            if let Some(item) = app.file_list.get_current_item() {
                                match item {
                                    FileSystemItem::File_(_) => {}
                                    FileSystemItem::Folder_(folder) => {
                                        if folder.file_filter_rules.len() > 0 && index < folder.file_filter_rules.len() {
                                            folder.file_filter_rules.remove(index);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('e') => {
                        if let Some(index) = app.file_item_list_filter.file_filter_list.selected() {
                            if let Some(item) = app.file_list.get_current_item() {
                                match item {
                                    FileSystemItem::File_(_) => {}
                                    FileSystemItem::Folder_(folder) => {
                                        let rule = &folder.file_filter_rules[index];
                                        app.file_item_list_filter.new_regex = rule.regex.to_owned();
                                        app.file_item_list_filter.new_deep = rule.deep.to_owned();
                                        app.file_item_list_filter.new_content = rule.content.to_owned();
                                        app.is_edit_file_filter_form_popup = true;
                                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Form));
                                    }
                                }
                            }
                        }
                    },
                    KeyCode::BackTab => app.change_mode(AppMode::FolderListFilter(FolderListFilter::List)),
                    KeyCode::Tab => app.change_mode(AppMode::FolderListPriority(FolderListPriority::List)),
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}