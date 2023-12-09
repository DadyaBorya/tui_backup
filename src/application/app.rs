use std::io::Stdout;

use crossterm::{
    terminal::{ enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode },
    execute,
    event::{ EnableMouseCapture, DisableMouseCapture },
};
use tui::{ backend::{ CrosstermBackend, Backend }, Terminal, style::Color };

use super::{ app_state::{ AppState, AppComponents }, app_ui, app_event, app_mode::AppMode };

pub static ACTIVE_BORDER_COLOR: Color = Color::Yellow;

pub struct App {
    pub state: AppState,
    pub components: AppComponents,
}

impl App {
    pub fn init() -> Result<Self, std::io::Error> {
        Ok(App { state: AppState::init()?, components: AppComponents::init()? })
    }

    pub fn change_mode(&mut self, mode: AppMode, prev_mode: AppMode) {
        self.state.prev_mode = prev_mode;
        self.state.mode = mode;
    }

    pub fn run_app<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>
    ) -> Result<(), std::io::Error> {
        terminal.draw(|f| app_ui::ui(self, f))?;

        if let Err(err) = app_event::event(self) {
            self.components.message_popup.state.edit("Error".to_string(), err.to_string());
            self.change_mode(AppMode::MessagePopup, self.state.mode.clone());
        }
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
