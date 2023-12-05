use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    Frame,
    layout::{ Alignment, Layout, Constraint },
    widgets::{ Block, BorderType, Borders, Clear, Paragraph },
};

use crate::{
    app_mode::{
        AppMode,
        FolderListFilter,
        FileFolderListFilter,
        FolderListPriority,
        FileFolderListPriority,
        FileListPriority,
    },
    app::App,
    popup::Popup,
};

pub struct HelpPopup {}

impl HelpPopup {
    pub fn get_help_by_app_mode<'a>(app_mode: &AppMode) -> Vec<(&str, &str)> {
        match app_mode {
            AppMode::Tab => {
                vec![
                    ("q", "Exit"),
                    ("<-", "Move left"),
                    ("->", "Move right"),
                    ("Space", "Select tab")
                ]
            }
            AppMode::FileList => {
                vec![
                    ("ESC", "Back"),
                    ("↑", "Move up"),
                    ("↓", "Move down"),
                    ("→", "Move into"),
                    ("←", "Move undo"),
                    ("Space", "Select"),
                    ("a", "Select all"),
                    ("s", "Select all into"),
                    ("f", "Filter"),
                    ("p", "Priority"),
                    ("c", "Save template")
                ]
            }
            AppMode::FolderListFilter(filter) => {
                match filter {
                    FolderListFilter::List => {
                        vec![
                            ("ESC", "Back"),
                            ("↑", "Move up"),
                            ("↓", "Move down"),
                            ("Tab", "Next"),
                            ("n", "New1"),
                            ("d", "Delete"),
                            ("e", "Edit")
                        ]
                    }
                    FolderListFilter::Submit => {
                        vec![("ESC", "Back"), ("BackTab", "Previous"), ("Enter", "Submit")]
                    }
                    _ => { vec![("ESC", "Back"), ("Tab", "Next"), ("BackTab", "Previous")] }
                }
            }
            AppMode::FileFolderListFilter(filter) => {
                match filter {
                    FileFolderListFilter::List => {
                        vec![
                            ("ESC", "Back"),
                            ("↑", "Move up"),
                            ("↓", "Move down"),
                            ("Tab", "Next"),
                            ("BackTab", "Previous"),
                            ("n", "New"),
                            ("d", "Delete"),
                            ("e", "Edit")
                        ]
                    }
                    FileFolderListFilter::Submit => {
                        vec![("ESC", "Back"), ("BackTab", "Previous"), ("Enter", "Submit")]
                    }
                    _ => { vec![("ESC", "Back"), ("Tab", "Next"), ("BackTab", "Previous")] }
                }
            }
            AppMode::FolderListPriority(filter) => {
                match filter {
                    FolderListPriority::List => {
                        vec![
                            ("ESC", "Back"),
                            ("↑", "Move up"),
                            ("↓", "Move down"),
                            ("Tab", "Next"),
                            ("BackTab", "Previous"),
                            ("n", "New"),
                            ("d", "Delete"),
                            ("e", "Edit")
                        ]
                    }
                    FolderListPriority::Submit => {
                        vec![("ESC", "Back"), ("BackTab", "Previous"), ("Enter", "Submit")]
                    }
                    _ => { vec![("ESC", "Back"), ("Tab", "Next"), ("BackTab", "Previous")] }
                }
            }
            AppMode::FileFolderListPriority(filter) => {
                match filter {
                    FileFolderListPriority::List => {
                        vec![
                            ("ESC", "Back"),
                            ("↑", "Move up"),
                            ("↓", "Move down"),
                            ("Tab", "Next"),
                            ("BackTab", "Previous"),
                            ("n", "New"),
                            ("d", "Delete"),
                            ("(e", "Edit")
                        ]
                    }
                    FileFolderListPriority::Submit => {
                        vec![("ESC", "Back"), ("BackTab", "Previous"), ("Enter", "Submit")]
                    }
                    _ => { vec![("ESC", "Back"), ("Tab", "Next"), ("BackTab", "Previous")] }
                }
            }
            AppMode::FileListPriority(filter) => {
                match filter {
                    FileListPriority::List => {
                        vec![
                            ("ESC", "Back"),
                            ("↑", "Move up"),
                            ("↓", "Move down"),
                            ("BackTab", "Previous"),
                            ("n", "New"),
                            ("d", "Delete"),
                            ("e", "Edit")
                        ]
                    }
                    FileListPriority::Submit => {
                        vec![("ESC", "Back"), ("BackTab", "Previous"), ("Enter", "Submit")]
                    }
                    _ => { vec![("ESC", "Back"), ("Tab", "Next"), ("BackTab", "Previous")] }
                }
            }
            _ => vec![],
        }
    }

    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
        let block = Block::default()
            .title("Help")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let rules = HelpPopup::get_help_by_app_mode(&app.prev_mode);

        let percent_y: u16 = 25 + ((5 * rules.len()) as u16);

        let area = Popup::centered_rect(20, percent_y, f.size());
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let chunks = Layout::default()
            .margin(2)
            .constraints([Constraint::Min(0)].as_ref())
            .split(area);

        let constraints: Vec<Constraint> = rules
            .iter()
            .map(|_| Constraint::Length(2))
            .collect();

        let rows = Layout::default().margin(1).constraints(constraints.as_ref()).split(chunks[0]);

        for (index, rule) in rules.iter().enumerate() {
            let text = format!("({}) {}", rule.0, rule.1);
            let paragraph = Paragraph::new(text).alignment(Alignment::Left);
            f.render_widget(paragraph, rows[index]);
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Esc => app.change_mode(app.prev_mode.clone()),
            _ => {}
        }
        Ok(())
    }
}
