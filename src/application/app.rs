use std::io::Stdout;

use crossterm::{
    terminal::{ enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode },
    execute,
    event::{ EnableMouseCapture, DisableMouseCapture },
};
use tui::{ backend::{ CrosstermBackend, Backend }, Terminal };

use super::{ app_state::AppState, app_ui, app_event };

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn init() -> Result<Self, std::io::Error> {
        Ok(App { state: AppState::init()? })
    }

    pub fn run_app<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>
    ) -> Result<(), std::io::Error> {
        terminal.draw(|f| app_ui::ui(self, f))?;
        app_event::event(self)?;
        Ok(())
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

    pub fn exit(&mut self) {
        self.state.exit = true;
    }
}
