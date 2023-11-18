use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Clear, Paragraph};
use crate::app::{App};
use crate::app_mode::AppMode;
use crate::popup::Popup;

#[derive(Debug, Clone)]
pub struct ErrorPopup {}

impl ErrorPopup {
    pub fn error_popup<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        if app.mode == AppMode::ErrorPopup {
            let block = Block::default()
                .title("Error")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let area = Popup::centered_rect(60, 25, f.size());
            f.render_widget(Clear, area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .margin(2)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2), Constraint::Length(2)
                    ].as_ref()
                ).split(area);

            let paragraph_text = match &app.error {
                None => { "unknown error" }
                Some(name) => { name }
            };

            let style = Style::default().fg(Color::Red);

            let text = Paragraph::new(paragraph_text)
                .style(style)
                .alignment(Alignment::Center);

            f.render_widget(text, chunks[0]);

            let keys_desc = Paragraph::new("Press (ESC) for close")
                .alignment(Alignment::Center);

            f.render_widget(keys_desc, chunks[1]);
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Esc => app.change_mode(AppMode::FileList),
            _ => {}
        }
        Ok(())
    }
}