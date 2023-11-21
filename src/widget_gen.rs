use tui::layout::Alignment;
use tui::style::Style;
use tui::widgets::{Block, Borders, BorderType, Paragraph};

pub struct WidgetGen {}

impl WidgetGen {
    pub fn form_input<'a>(title: &'a str, value: &'a str, style: Style) -> Paragraph<'a> {
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        Paragraph::new(value)
            .block(block)
            .style(style)
    }

    pub fn form_button(value: &str, style: Style) -> Paragraph {
        let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

        Paragraph::new(value)
            .alignment(Alignment::Center)
            .block(block)
            .style(style)
    }
}