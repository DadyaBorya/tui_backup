use std::io::Stdout;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::Color,
    Terminal,
};

use crate::models::config::Config;

use super::{
    mode::AppMode,
    state::{AppComponents, AppState},
};

pub static ACTIVE_BORDER_COLOR: Color = Color::Yellow;

pub struct App {
    pub state: AppState,
    pub components: AppComponents,
    pub config: Config,
}

impl App {
    pub fn init() -> Result<Self, std::io::Error> {
        let config = Config::init()?;

        Ok(App {
            state: AppState::init()?,
            components: AppComponents::init(&config)?,
            config,
        })
    }

    pub fn change_mode(&mut self, mode: AppMode, prev_mode: AppMode) {
        self.state.prev_mode = prev_mode;
        self.state.mode = mode;
    }

    pub fn run_app<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), std::io::Error> {
        terminal.draw(|f| App::ui(self, f))?;

        if let Err(err) = App::event(self) {
            self.components
                .message_popup
                .state
                .edit("Error".to_string(), err.to_string(), 50, 50);
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
        terminal: &mut CrosstermBackend<Stdout>,
    ) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        execute!(terminal, LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) {
        self.state.exit = true;
    }
}
