use crossterm::event::KeyCode;

use crate::application::app::App;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let tabs = &mut app.state.components.tabs;

    match key_code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Right => tabs.next(),
        KeyCode::Left => tabs.previous(),
        _ => {}
    }
    Ok(())
}
