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


pub enum AppMode {
    Tab,
    FileList
}

pub struct App<'a> {
    pub mode: AppMode,
    pub tabs: TabC<'a>,
    pub file_list: FileList,
    pub exit: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(App {
            mode: AppMode::Tab,
            tabs: TabC::new(),
            file_list: FileList::new()?,
            exit: false,
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



