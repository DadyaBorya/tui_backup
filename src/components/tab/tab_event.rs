use crossterm::event::KeyCode;

use crate::application::app::App;

use super::tab_component::TabComponent;

pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
    let tabs = &mut app.components.tabs;
    match key_code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Right => tabs.next(),
        KeyCode::Left => tabs.previous(),
        KeyCode::Enter => TabComponent::select_tab(app),
        _ => {}
    }
    Ok(())
}
