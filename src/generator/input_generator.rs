use tui::{
    widgets::{ Block, Borders, BorderType, Paragraph },
    style::{ Style, Color },
    layout::Alignment,
};

use crate::application::app::ACTIVE_BORDER_COLOR;

pub fn input<'a>(
    title: String,
    value: String,
    place_holder: String,
    is_selected: bool
) -> Paragraph<'a> {
    let style = match is_selected {
        true => Style::default().fg(ACTIVE_BORDER_COLOR),
        false => Style::default().fg(Color::White),
    };

    let block = Block::default()
        .title(title)
        .style(style)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    match value.is_empty() {
        true => {
            let style = Style::default().fg(Color::Rgb(220, 220, 220));
            Paragraph::new(place_holder).style(style).block(block)
        }
        false => {
            let style = Style::default().fg(Color::White);
            Paragraph::new(value).style(style).block(block)
        }
    }
}

pub fn button<'a>(title: String, is_selected: bool) -> Paragraph<'a> {
    let style = match is_selected {
        true => Style::default().fg(ACTIVE_BORDER_COLOR),
        false => Style::default().fg(Color::White),
    };

    let block = Block::default()
        .style(style)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    Paragraph::new(title).block(block).alignment(Alignment::Center)
}
