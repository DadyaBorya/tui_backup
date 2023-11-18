use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, BorderType, List, ListItem, ListState};
use crate::app::{App};
use crate::app_mode::{AppMode, FileFolderListFilter, FolderListFilter};
use crate::file_system::FileSystemItem;

#[derive(Debug, Clone)]
pub struct FolderPriority {
    pub regex: String,
    pub deep: String,
    pub priority: String,
}

#[derive(Debug, Clone)]
pub struct FileFolderPriority {
    pub regex: String,
    pub deep: String,
    pub content: String,
    pub priority: String,
}

#[derive(Debug, Clone)]
pub struct FilePriority {
    pub regex: String,
    pub priority: String,
}

pub struct FileItemListPriority {
    pub file_priority_rules: Vec<FilePriority>,
    pub file_folder_priority_rules: Vec<FileFolderPriority>,
    pub folder_priority_rules: Vec<FolderPriority>,
    pub file_priority_list: ListState,
    pub file_folder_priority_list: ListState,
    pub folder_priority_list: ListState,
    pub new_regex: String,
    pub new_priority: String,
    pub new_content: String,
    pub new_deep: String,
}

impl FileItemListPriority {
    pub fn new() -> Self {
        FileItemListPriority {
            file_priority_rules: vec![],
            file_folder_priority_rules: vec![],
            folder_priority_rules: vec![],
            file_priority_list: ListState::default(),
            file_folder_priority_list: ListState::default(),
            folder_priority_list: ListState::default(),
            new_regex: "".to_string(),
            new_priority: "".to_string(),
            new_content: "".to_string(),
            new_deep: "".to_string(),
        }
    }

    pub fn clean_inputs(&mut self) {
        self.new_regex.clear();
        self.new_priority.clear();
        self.new_content.clear();
        self.new_deep.clear();
    }

    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        if let Some(item) = app.file_list.get_current_item() {
            match item {
                FileSystemItem::File_(file) => {
                    let list_chunk = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Percentage(50), Constraint::Percentage(50)
                            ].as_ref()
                        ).split(chunks[0]);

                    app.file_item_list_priority.file_priority_rules = file.file_priority_rules.to_owned();

                    let file_items: Vec<ListItem> = file.file_priority_rules.to_owned().into_iter()
                        .map(|item| {
                            ListItem::new(format!("{} ({})", item.regex, item.priority))
                        }).collect();

                    let file_list = List::new(file_items)
                        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("File Priority").title_alignment(Alignment::Center))
                        .style(match app.mode {
                            AppMode::FileFolderListFilter(FileFolderListFilter::List) => Style::default().fg(Color::Yellow),
                            _ => Style::default(),
                        })
                        .highlight_symbol("->")
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                    f.render_stateful_widget(file_list, list_chunk[0], &mut app.file_item_list_priority.file_priority_list);
                }
                FileSystemItem::Folder_(folder) => {
                    let list_chunk = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Percentage(50), Constraint::Percentage(50)
                            ].as_ref()
                        ).split(chunks[1]);

                    app.file_item_list_priority.folder_priority_rules = folder.folder_priority_rules.to_owned();
                    app.file_item_list_priority.file_folder_priority_rules = folder.file_priority_rules.to_owned();

                    let folder_items: Vec<ListItem> = folder.folder_priority_rules.to_owned().into_iter()
                        .map(|item| {
                            ListItem::new(format!("{} ({} -> {})", item.regex, item.deep, item.priority))
                        }).collect();

                    let folder_list = List::new(folder_items)
                        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Folder Priority").title_alignment(Alignment::Center))
                        .style(match app.mode {
                            AppMode::FolderListFilter(FolderListFilter::List) => Style::default().fg(Color::Yellow),
                            _ => Style::default()
                        })
                        .highlight_symbol("->")
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                    f.render_stateful_widget(folder_list, list_chunk[0], &mut app.file_item_list_priority.folder_priority_list);

                    let file_items: Vec<ListItem> = folder.file_priority_rules.to_owned().into_iter()
                        .map(|item| {
                            ListItem::new(format!("{} ({} -> {})\n{}", item.regex, item.deep, item.priority, item.content))
                        }).collect();

                    let file_list = List::new(file_items)
                        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("File Priority").title_alignment(Alignment::Center))
                        .style(match app.mode {
                            AppMode::FileFolderListFilter(FileFolderListFilter::List) => Style::default().fg(Color::Yellow),
                            _ => Style::default(),
                        })
                        .highlight_symbol("->")
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                    f.render_stateful_widget(file_list, list_chunk[1], &mut app.file_item_list_priority.file_folder_priority_list);
                }
            }
        }
    }
}