use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::widgets::{Block, Borders, BorderType, Paragraph};
use crate::app::App;
use crate::app_mode::{AppMode, FileFolderListFilter, FileFolderListPriority, FileListPriority, FolderListFilter, FolderListPriority};

pub struct HelpBlock {}

impl HelpBlock {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App, chunks: &Vec<Rect>) {
        let block = Block::default()
            .title("Help")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = match &app.mode {
            AppMode::Tab => {
                "(q) Exit, (<-) Move left, (->) Move right, (SPACE) Select tab"
            }
            AppMode::FileList => {
                "(ESC) Back, (↑) Move up, (↓) Move down, (→) Move into, (←) Move undo, (SPACE) Select, (a) Select all, (s) Select all into, (f) Filter, (p) Priority"
            }
            AppMode::FolderListFilter(filter) => {
                match filter {
                    FolderListFilter::List => {
                        "(ESC) Back, (Tab) Next, (↑) Move up, (↓) Move down, (n) New, (d) Delete, (e) Edit"
                    },
                    FolderListFilter::Submit => {
                        "(ESC) Back, (BackTab) Previous, (Enter) Submit"
                    }
                   _ => "(ESC) Back, (Tab) Next, (BackTab) Previous"
                }
            }
            AppMode::FileFolderListFilter(filter) => {
                match filter {
                    FileFolderListFilter::List => {
                        "(ESC) Back, (Tab) Next, (BackTab) Previous, (↑) Move up, (↓) Move down, (n) New, (d) Delete, (e) Edit"
                    },
                    FileFolderListFilter::Submit => {
                        "(ESC) Back, (BackTab) Previous, (Enter) Submit"
                    }
                    _ => "(ESC) Back, (Tab) Next, (BackTab) Previous"
                }
            }
            AppMode::FolderListPriority(filter) => {
                match filter {
                    FolderListPriority::List => {
                        "(ESC) Back, (Tab) Next, (BackTab) Previous, (↑) Move up, (↓) Move down, (n) New, (d) Delete, (e) Edit"
                    },
                    FolderListPriority::Submit => {
                        "(ESC) Back, (BackTab) Previous, (Enter) Submit"
                    }
                    _ => "(ESC) Back, (Tab) Next, (BackTab) Previous"
                }
            }
            AppMode::FileFolderListPriority(filter) => {
                match filter {
                    FileFolderListPriority::List => {
                        "(ESC) Back, (BackTab) Previous, (↑) Move up, (↓) Move down, (n) New, (d) Delete, (e) Edit"
                    },
                    FileFolderListPriority::Submit => {
                        "(ESC) Back, (BackTab) Previous, (Enter) Submit"
                    }
                    _ => "(ESC) Back, (Tab) Next, (BackTab) Previous"
                }
            }
            AppMode::FileListPriority(filter) => {
                match filter {
                    FileListPriority::List => {
                        "(ESC) Back, (↑) Move up, (↓) Move down, (n) New, (d) Delete, (e) Edit"
                    },
                    FileListPriority::Submit => {
                        "(ESC) Back, (BackTab) Previous, (Enter) Submit"
                    }
                    _ => "(ESC) Back, (Tab) Next, (BackTab) Previous"
                }
            }
            AppMode::ErrorPopup => "(ESC) Back"
        };

        let par = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(block);

        f.render_widget(par, chunks[2]);
    }
}