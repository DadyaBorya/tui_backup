use crossterm::event::KeyCode;
use serde::{Serialize, Deserialize};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{ Alignment, Constraint, Direction, Layout, Rect };
use tui::style::{ Color, Modifier, Style };
use tui::widgets::{ Block, Borders, BorderType, List, ListItem, ListState };
use crate::app::App;
use crate::app_mode::{
    AppMode,
    FileFolderListFilter,
    FileFolderListPriority,
    FileListPriority,
    FolderListPriority,
};
use crate::file_system::FileSystemItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderPriority {
    pub regex: String,
    pub deep: String,
    pub priority: String,
}

impl FolderPriority {
    pub fn new(regex: String, deep: String, priority: String) -> Self {
        FolderPriority {
            regex,
            deep,
            priority,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFolderPriority {
    pub regex: String,
    pub deep: String,
    pub content: String,
    pub priority: String,
}

impl FileFolderPriority {
    pub fn new(regex: String, deep: String, content: String, priority: String) -> Self {
        FileFolderPriority {
            regex,
            deep,
            content,
            priority,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePriority {
    pub content: String,
    pub priority: String,
}

impl FilePriority {
    pub fn new(content: String, priority: String) -> Self {
        FilePriority {
            content,
            priority,
        }
    }
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
                            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()
                        )
                        .split(chunks[0]);

                    app.file_item_list_priority.file_priority_rules =
                        file.file_priority_rules.to_owned();

                    let file_items: Vec<ListItem> = file.file_priority_rules
                        .to_owned()
                        .into_iter()
                        .map(|item| {
                            ListItem::new(
                                format!("priority: {}\ncontent: {}", item.content, item.priority)
                            )
                        })
                        .collect();

                    let file_list = List::new(file_items)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .title("File Priority")
                                .title_alignment(Alignment::Center)
                        )
                        .style(match app.mode {
                            AppMode::FileListPriority(FileListPriority::List) =>
                                Style::default().fg(Color::Yellow),
                            _ => Style::default(),
                        })
                        .highlight_symbol("->")
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                    f.render_stateful_widget(
                        file_list,
                        list_chunk[0],
                        &mut app.file_item_list_priority.file_priority_list
                    );
                }
                FileSystemItem::Folder_(folder) => {
                    let list_chunk = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()
                        )
                        .split(chunks[1]);

                    app.file_item_list_priority.folder_priority_rules =
                        folder.folder_priority_rules.to_owned();
                    app.file_item_list_priority.file_folder_priority_rules =
                        folder.file_priority_rules.to_owned();

                    let folder_items: Vec<ListItem> = folder.folder_priority_rules
                        .to_owned()
                        .into_iter()
                        .map(|item| {
                            ListItem::new(
                                format!(
                                    "regex: {}; deep: {}; priority: {}",
                                    item.regex,
                                    item.deep,
                                    item.priority
                                )
                            )
                        })
                        .collect();

                    let folder_list = List::new(folder_items)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .title("Folder Priority")
                                .title_alignment(Alignment::Center)
                        )
                        .style(match app.mode {
                            AppMode::FolderListPriority(FolderListPriority::List) =>
                                Style::default().fg(Color::Yellow),
                            _ => Style::default(),
                        })
                        .highlight_symbol("->")
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                    f.render_stateful_widget(
                        folder_list,
                        list_chunk[0],
                        &mut app.file_item_list_priority.folder_priority_list
                    );

                    let file_items: Vec<ListItem> = folder.file_priority_rules
                        .to_owned()
                        .into_iter()
                        .map(|item| {
                            ListItem::new(
                                format!(
                                    "regex: {}; deep: {}; priority: {}\ncontent: {}",
                                    item.regex,
                                    item.deep,
                                    item.priority,
                                    item.content
                                )
                            )
                        })
                        .collect();

                    let file_list = List::new(file_items)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .title("File Priority")
                                .title_alignment(Alignment::Center)
                        )
                        .style(match app.mode {
                            AppMode::FileFolderListPriority(FileFolderListPriority::List) =>
                                Style::default().fg(Color::Yellow),
                            _ => Style::default(),
                        })
                        .highlight_symbol("->")
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                    f.render_stateful_widget(
                        file_list,
                        list_chunk[1],
                        &mut app.file_item_list_priority.file_folder_priority_list
                    );
                }
            }
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FolderListPriority(FolderListPriority::List) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileList);
                        app.file_item_list_priority.folder_priority_list.select(None);
                    }
                    KeyCode::BackTab =>
                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::List)),
                    KeyCode::Tab =>
                        app.change_mode(
                            AppMode::FileFolderListPriority(FileFolderListPriority::List)
                        ),
                    KeyCode::Up => {
                        let priority = &mut app.file_item_list_priority;

                        if priority.folder_priority_rules.is_empty() {
                            priority.folder_priority_list.select(None);
                        } else {
                            let selected = match priority.folder_priority_list.selected() {
                                Some(v) => {
                                    if v == 0 { Some(v) } else { Some(v - 1) }
                                }
                                None => { Some(0) }
                            };

                            priority.folder_priority_list.select(selected);
                        }
                    }
                    KeyCode::Down => {
                        let priority = &mut app.file_item_list_priority;

                        if priority.folder_priority_rules.is_empty() {
                            priority.folder_priority_list.select(None);
                        } else {
                            let selected = match priority.folder_priority_list.selected() {
                                Some(v) => {
                                    if v == priority.folder_priority_rules.len() - 1 {
                                        Some(v)
                                    } else {
                                        Some(v + 1)
                                    }
                                }
                                None => { Some(0) }
                            };

                            priority.folder_priority_list.select(selected);
                        }
                    }
                    KeyCode::Char('n') =>
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Form)),
                    KeyCode::Char('d') => {
                        if
                            let Some(index) =
                                app.file_item_list_priority.folder_priority_list.selected()
                        {
                            if let Some(item) = app.file_list.get_current_item() {
                                if let FileSystemItem::Folder_(folder) = item {
                                    if
                                        folder.folder_priority_rules.len() > 0 &&
                                        index < folder.folder_priority_rules.len()
                                    {
                                        let folder_priority =
                                            folder.folder_priority_rules[index].to_owned();
                                        folder.delete_priority_by_folder(folder_priority);
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('e') => {
                        if
                            let Some(index) =
                                app.file_item_list_priority.folder_priority_list.selected()
                        {
                            if let Some(item) = app.file_list.get_current_item() {
                                if let FileSystemItem::Folder_(folder) = item {
                                    let rule = &folder.folder_priority_rules[index];
                                    let priority = &mut app.file_item_list_priority;
                                    priority.new_regex = rule.regex.to_owned();
                                    priority.new_deep = rule.deep.to_owned();
                                    priority.new_priority = rule.priority.to_owned();
                                    app.is_edit_folder_priority_form_popup = true;
                                    app.change_mode(
                                        AppMode::FolderListPriority(FolderListPriority::Form)
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Char('h') => {
                        app.prev_mode = AppMode::FolderListPriority(FolderListPriority::List);
                        app.change_mode(AppMode::HelpPopup);
                    }
                    _ => {}
                }
            }
            AppMode::FileFolderListPriority(FileFolderListPriority::List) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileList);
                        app.file_item_list_priority.file_folder_priority_list.select(None);
                    }
                    KeyCode::BackTab =>
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::List)),
                    KeyCode::Up => {
                        let priority = &mut app.file_item_list_priority;

                        if priority.file_folder_priority_rules.is_empty() {
                            priority.file_folder_priority_list.select(None);
                        } else {
                            let selected = match priority.file_folder_priority_list.selected() {
                                Some(v) => {
                                    if v == 0 { Some(v) } else { Some(v - 1) }
                                }
                                None => { Some(0) }
                            };

                            priority.file_folder_priority_list.select(selected);
                        }
                    }
                    KeyCode::Down => {
                        let priority = &mut app.file_item_list_priority;

                        if priority.file_folder_priority_rules.is_empty() {
                            priority.file_folder_priority_list.select(None);
                        } else {
                            let selected = match priority.file_folder_priority_list.selected() {
                                Some(v) => {
                                    if v == priority.file_folder_priority_rules.len() - 1 {
                                        Some(v)
                                    } else {
                                        Some(v + 1)
                                    }
                                }
                                None => { Some(0) }
                            };

                            priority.file_folder_priority_list.select(selected);
                        }
                    }
                    KeyCode::Char('n') =>
                        app.change_mode(
                            AppMode::FileFolderListPriority(FileFolderListPriority::Form)
                        ),
                    KeyCode::Char('d') => {
                        if
                            let Some(index) =
                                app.file_item_list_priority.file_folder_priority_list.selected()
                        {
                            if let Some(item) = app.file_list.get_current_item() {
                                if let FileSystemItem::Folder_(folder) = item {
                                    if
                                        folder.file_priority_rules.len() > 0 &&
                                        index < folder.file_priority_rules.len()
                                    {
                                        let folder_priority =
                                            folder.file_priority_rules[index].to_owned();
                                        folder.delete_priority_by_file_folder(folder_priority);
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('e') => {
                        if
                            let Some(index) =
                                app.file_item_list_priority.file_folder_priority_list.selected()
                        {
                            if let Some(item) = app.file_list.get_current_item() {
                                if let FileSystemItem::Folder_(folder) = item {
                                    let rule = &folder.file_priority_rules[index];
                                    let priority = &mut app.file_item_list_priority;
                                    priority.new_regex = rule.regex.to_owned();
                                    priority.new_deep = rule.deep.to_owned();
                                    priority.new_priority = rule.priority.to_owned();
                                    priority.new_content = rule.content.to_owned();
                                    app.is_edit_file_folder_priority_form_popup = true;
                                    app.change_mode(
                                        AppMode::FileFolderListPriority(
                                            FileFolderListPriority::Form
                                        )
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Char('h') => {
                        app.prev_mode = AppMode::FileFolderListPriority(
                            FileFolderListPriority::List
                        );
                        app.change_mode(AppMode::HelpPopup);
                    }
                    _ => {}
                }
            }
            AppMode::FileListPriority(FileListPriority::List) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileList);
                        app.file_item_list_priority.file_priority_list.select(None);
                    }
                    KeyCode::Up => {
                        let priority = &mut app.file_item_list_priority;

                        if priority.file_priority_rules.is_empty() {
                            priority.file_priority_list.select(None);
                        } else {
                            let selected = match priority.file_priority_list.selected() {
                                Some(v) => {
                                    if v == 0 { Some(v) } else { Some(v - 1) }
                                }
                                None => { Some(0) }
                            };

                            priority.file_priority_list.select(selected);
                        }
                    }
                    KeyCode::Down => {
                        let priority = &mut app.file_item_list_priority;

                        if priority.file_priority_rules.is_empty() {
                            priority.file_priority_list.select(None);
                        } else {
                            let selected = match priority.file_priority_list.selected() {
                                Some(v) => {
                                    if v == priority.file_priority_rules.len() - 1 {
                                        Some(v)
                                    } else {
                                        Some(v + 1)
                                    }
                                }
                                None => { Some(0) }
                            };

                            priority.file_priority_list.select(selected);
                        }
                    }
                    KeyCode::Char('n') =>
                        app.change_mode(AppMode::FileListPriority(FileListPriority::Form)),
                    KeyCode::Char('d') => {
                        if
                            let Some(index) =
                                app.file_item_list_priority.file_priority_list.selected()
                        {
                            if let Some(item) = app.file_list.get_current_item() {
                                if let FileSystemItem::File_(file) = item {
                                    if
                                        file.file_priority_rules.len() > 0 &&
                                        index < file.file_priority_rules.len()
                                    {
                                        file.file_priority_rules.remove(index);
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('e') => {
                        if
                            let Some(index) =
                                app.file_item_list_priority.file_priority_list.selected()
                        {
                            if let Some(item) = app.file_list.get_current_item() {
                                if let FileSystemItem::File_(file) = item {
                                    let rule = &file.file_priority_rules[index];
                                    let priority = &mut app.file_item_list_priority;
                                    priority.new_priority = rule.priority.to_owned();
                                    priority.new_content = rule.content.to_owned();
                                    app.is_edit_file_priority_form_popup = true;
                                    app.change_mode(
                                        AppMode::FileListPriority(FileListPriority::Form)
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Char('h') => {
                        app.prev_mode = AppMode::FileListPriority(FileListPriority::List);
                        app.change_mode(AppMode::HelpPopup);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}
