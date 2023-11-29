use std::error::Error;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crate::app::{App};

mod tab_c;
mod app;
mod file_system;
mod file_service;
mod file_list;
mod popup;
mod file_item_list_filter;
mod file_item_list_priority;
mod error_popup;
mod folder_filter_form_popup;
mod file_filter_form_popup;
mod app_mode;
mod folder_priority_form_popup;
mod widget_gen;
mod file_folder_priority_form_popup;
mod file_list_priority_form_popup;
mod help_block;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new()?;
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

        if app.exit {
            app.disable_alternative_screen(terminal.backend_mut())?;
            println!("Bye, bye!");
            break;
        }
    }

    Ok(())
}
