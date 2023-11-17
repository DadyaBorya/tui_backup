use std::io::Stdout;
use crossterm::{event, execute};
use crossterm::event::Event::Key;
use crossterm::event::{DisableMouseCapture, KeyEventKind};
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};
use tui::layout::{Constraint, Direction, Layout};
use crate::file_list::FileList;
use crate::tab_c::TabC;
use crossterm::event::{EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::error_popup::ErrorPopup;
use crate::file_filter_form_popup::FileFilterFormPopup;
use crate::file_list_filter::FileListFilter;
use crate::folder_filter_form_popup::FolderFilterFormPopup;


#[derive(PartialEq)]
pub enum AppMode {
    Tab,
    FileList,
    ErrorPopup,

    FolderListFilter,
    FolderListFilterForm,
    FolderListFilterFormRegex,
    FolderListFilterFormDeep,
    FolderListFilterFormSubmit,

    FileListFilter,
    FileListFilterForm,
    FileListFilterFormRegex,
    FileListFilterFormDeep,
    FileListFilterFormContent,
    FileListFilterFormSubmit,
}

pub struct App<'a> {
    pub mode: AppMode,
    pub tabs: TabC<'a>,
    pub file_list: FileList,
    pub file_list_filter: FileListFilter,
    pub is_folder_filter_form_popup: bool,
    pub is_edit_folder_filter_form_popup: bool,
    pub is_file_filter_form_popup: bool,
    pub is_edit_file_filter_form_popup: bool,
    pub error: Option<String>,
    pub exit: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(App {
            mode: AppMode::Tab,
            tabs: TabC::new(),
            file_list: FileList::new()?,
            file_list_filter: FileListFilter::new(),
            exit: false,
            error: None,
            is_folder_filter_form_popup: false,
            is_edit_folder_filter_form_popup: false,
            is_edit_file_filter_form_popup: false,
            is_file_filter_form_popup: false
        })
    }

    pub fn change_mode(&mut self, mode: AppMode) {
        self.mode = mode;
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
        terminal.draw(|f| self.ui(f))?;
        self.event()?;
        Ok(())
    }

    pub fn event(&mut self) -> Result<(), std::io::Error> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.mode {
                        AppMode::Tab => TabC::event(self, key.code)?,
                        AppMode::FileList => FileList::event(self, key.code)?,
                        AppMode::ErrorPopup => ErrorPopup::event(self, key.code)?,

                        AppMode::FolderListFilter => FileListFilter::event(self, key.code)?,
                        AppMode::FolderListFilterForm => FolderFilterFormPopup::event(self, key.code)?,
                        AppMode::FolderListFilterFormRegex => FolderFilterFormPopup::event(self, key.code)?,
                        AppMode::FolderListFilterFormDeep => FolderFilterFormPopup::event(self, key.code)?,
                        AppMode::FolderListFilterFormSubmit => FolderFilterFormPopup::event(self, key.code)?,

                        AppMode::FileListFilter => FileListFilter::event(self, key.code)?,
                        AppMode::FileListFilterForm => FileFilterFormPopup::event(self, key.code)?,
                        AppMode::FileListFilterFormRegex => FileFilterFormPopup::event(self, key.code)?,
                        AppMode::FileListFilterFormDeep => FileFilterFormPopup::event(self, key.code)?,
                        AppMode::FileListFilterFormContent => FileFilterFormPopup::event(self, key.code)?,
                        AppMode::FileListFilterFormSubmit => FileFilterFormPopup::event(self, key.code)?,
                    }
                }
            }
        }

        Ok(())
    }

    pub fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        TabC::ui(self, f, &chunks);

        match self.tabs.index {
            0 => FileList::ui(self, f, &chunks),
            _ => {  },
        };

        ErrorPopup::error_popup(f, self);
        FolderFilterFormPopup::ui(f, self);
        FileFilterFormPopup::ui(f, self);
    }

    pub fn execute_alternative_screen(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture)?;
        Ok(())
    }

    pub fn disable_alternative_screen(&self, terminal: &mut CrosstermBackend<Stdout>) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        execute!(terminal, LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}



