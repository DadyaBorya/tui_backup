use std::error::Error;
use crossterm::execute;
use crossterm::event::{DisableMouseCapture};
use crossterm::terminal::{LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crate::app::App;

mod tab_c;
mod app;
mod file_system;
mod file_service;
mod file_list;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new()?;

    app.execute_alternative_screen()?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    while true {
        let res = app.run_app(&mut terminal);

        if res.is_err() {
            println!("{:?}", res);
            app.exit = true;
        }

        if app.exit {
            app.disable_alternative_screen()?;
            execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
            break;
        }
    }

    Ok(())
}
