use tui::{
    backend::Backend,
    Frame,
    widgets::{ Block, Borders, BorderType, Clear, Paragraph },
    layout::{ Alignment, Layout, Direction, Constraint },
    style::Style,
};

use crate::{ application::app::{ App, ACTIVE_BORDER_COLOR }, generator::popup };

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let popup_state = &app.components.message_popup.state;

    let block = Block::default()
        .title(popup_state.title.as_str())
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACTIVE_BORDER_COLOR))
        .border_type(BorderType::Rounded);

    let area = popup::centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3)].as_ref())
        .split(area);

    let text = Paragraph::new(popup_state.message.as_str()).alignment(Alignment::Center);

    f.render_widget(text, chunks[0]);
}
