use tui::{
    backend::Backend,
    Frame,
    layout::{ Rect, Alignment },
    widgets::{ Block, Borders, BorderType, Paragraph },
};

use crate::application::{ app::App, app_mode::AppMode };

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    let helper_text = get_helper_text(&app);

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let paragraph = Paragraph::new(helper_text).block(block).alignment(Alignment::Center);

    f.render_widget(paragraph, chunks[2]);
}

fn get_helper_text(app: &App) -> &'static str {
    let mode = &app.state.mode;

    match mode {
        AppMode::Tab => app.state.components.tabs.get_helper_text(),
        _ => "",
    }
}
