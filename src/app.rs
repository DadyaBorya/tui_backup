use std::io::Stdout;
use crossterm::{ event, execute };
use crossterm::event::Event::Key;
use crossterm::event::{ DisableMouseCapture, KeyEventKind };
use tui::backend::{ Backend, CrosstermBackend };
use tui::{ Frame, Terminal };
use tui::layout::{ Constraint, Direction, Layout };
use crate::create_template_popup::CreateTemplatePopup;
use crate::file_list::FileList;
use crate::help_popup::HelpPopup;
use crate::tab_c::TabC;
use crate::template::Template;
use crate::template_list::TemplateList;
use crossterm::event::EnableMouseCapture;
use crossterm::terminal::{
    disable_raw_mode,
    enable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crate::app_mode::{
    AppMode,
    FileFolderListFilter,
    FileFolderListPriority,
    FileListPriority,
    FolderListFilter,
    FolderListPriority,
};
use crate::error_popup::ErrorPopup;
use crate::file_filter_form_popup::FileFilterFormPopup;
use crate::file_folder_priority_form_popup::FileFolderPriorityFormPopup;
use crate::file_item_list_filter::FileItemListFilter;
use crate::file_item_list_priority::FileItemListPriority;
use crate::file_list_priority_form_popup::FileListPriorityFormPopup;
use crate::folder_filter_form_popup::FolderFilterFormPopup;
use crate::folder_priority_form_popup::FolderPriorityFormPopup;

pub struct App<'a> {
    pub mode: AppMode,
    pub prev_mode: AppMode,
    pub tabs: TabC<'a>,
    pub file_list: FileList,
    pub file_item_list_filter: FileItemListFilter,
    pub file_item_list_priority: FileItemListPriority,
    pub template_list: TemplateList,
    pub create_template: Template,
    pub is_edit_folder_filter_form_popup: bool,
    pub is_edit_file_filter_form_popup: bool,
    pub is_edit_folder_priority_form_popup: bool,
    pub is_edit_file_folder_priority_form_popup: bool,
    pub is_edit_file_priority_form_popup: bool,
    pub is_edit_template_list: bool,
    pub error: Option<String>,
    pub exit: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(App {
            mode: AppMode::Tab,
            tabs: TabC::new(),
            file_list: FileList::new()?,
            file_item_list_filter: FileItemListFilter::new(),
            file_item_list_priority: FileItemListPriority::new(),
            template_list: TemplateList::new(),
            exit: false,
            error: None,
            is_edit_folder_filter_form_popup: false,
            is_edit_file_filter_form_popup: false,
            is_edit_file_folder_priority_form_popup: false,
            is_edit_file_priority_form_popup: false,
            is_edit_folder_priority_form_popup: false,
            is_edit_template_list: false,
            prev_mode: AppMode::Tab,
            create_template: Template::new("".to_string()),
        })
    }

    pub fn change_mode(&mut self, mode: AppMode) {
        self.mode = mode;
    }

    pub fn run_app<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>
    ) -> Result<(), std::io::Error> {
        terminal.draw(|f| self.ui(f))?;
        self.event()?;
        Ok(())
    }

    pub fn event(&mut self) -> Result<(), std::io::Error> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match &mut self.mode {
                        AppMode::Tab => TabC::event(self, key.code)?,
                        AppMode::FileList => FileList::event(self, key.code)?,
                        AppMode::ErrorPopup => ErrorPopup::event(self, key.code)?,
                        AppMode::FolderListFilter(filter) =>
                            match filter {
                                FolderListFilter::List =>
                                    FileItemListFilter::event(self, key.code)?,
                                _ => FolderFilterFormPopup::event(self, key.code)?,
                            }

                        AppMode::FileFolderListFilter(filter) =>
                            match filter {
                                FileFolderListFilter::List =>
                                    FileItemListFilter::event(self, key.code)?,
                                _ => FileFilterFormPopup::event(self, key.code)?,
                            }

                        AppMode::FolderListPriority(priority) =>
                            match priority {
                                FolderListPriority::List =>
                                    FileItemListPriority::event(self, key.code)?,
                                _ => FolderPriorityFormPopup::event(self, key.code)?,
                            }

                        AppMode::FileFolderListPriority(priority) =>
                            match priority {
                                FileFolderListPriority::List =>
                                    FileItemListPriority::event(self, key.code)?,
                                _ => FileFolderPriorityFormPopup::event(self, key.code)?,
                            }

                        AppMode::FileListPriority(priority) =>
                            match priority {
                                FileListPriority::List =>
                                    FileItemListPriority::event(self, key.code)?,
                                _ => FileListPriorityFormPopup::event(self, key.code)?,
                            }
                        AppMode::HelpPopup => HelpPopup::event(self, key.code)?,
                        AppMode::CreateTemplate(_) => CreateTemplatePopup::event(self, key.code)?,
                        AppMode::TemplateList => TemplateList::event(self, key.code)?,
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
            1 => {
                TemplateList::ui(self, f, &chunks);
            }
            _ => {}
        }

        match self.mode {
            AppMode::FolderListFilter(_) => FolderFilterFormPopup::ui(f, self),
            AppMode::FileFolderListFilter(_) => FileFilterFormPopup::ui(f, self),
            AppMode::FolderListPriority(_) => FolderPriorityFormPopup::ui(f, self),
            AppMode::FileFolderListPriority(_) => FileFolderPriorityFormPopup::ui(f, self),
            AppMode::FileListPriority(_) => FileListPriorityFormPopup::ui(f, self),
            AppMode::HelpPopup => HelpPopup::ui(f, self),
            AppMode::CreateTemplate(_) => CreateTemplatePopup::ui(f, self),
            _ => {}
        }
        ErrorPopup::error_popup(f, self);
    }

    pub fn execute_alternative_screen(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    pub fn disable_alternative_screen(
        &self,
        terminal: &mut CrosstermBackend<Stdout>
    ) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        execute!(terminal, LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}
