use std::error::Error;
use application::app::App;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod components;
mod services;
mod models;
mod application;
mod generator;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::init()?;
    app.execute_alternative_screen()?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        let res = app.run_app(&mut terminal);

        if res.is_err() {
            app.disable_alternative_screen(terminal.backend_mut())?;
            println!("{:?}", res);
            break;
        }

        if app.state.exit {
            app.disable_alternative_screen(terminal.backend_mut())?;
            println!("Bye, bye!");
            break;
        }
    }

    Ok(())
}
