use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Rect, Layout, Alignment};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Clear, Paragraph};
use crate::app::{App, AppMode};

#[derive(Clone, Debug)]
pub struct Popup {}

impl Popup {
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
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Length(2)
                    ].as_ref()
                )
                .split(area);

            let paragraph_text = match &app.error {
                None => {"unknown error"}
                Some(name) => {name}
            };

            let text = Paragraph::new(paragraph_text)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            f.render_widget(text,  chunks[0]);

            let keys_desc = Paragraph::new("Press (ESC) for close")
                .alignment(Alignment::Center);

            f.render_widget(keys_desc, chunks[1]);
        }
    }

    pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                    .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                    .as_ref(),
            )
            .split(popup_layout[1])[1]
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Esc => {
                app.change_mode(AppMode::FileList)
            }
            _ => {}
        }

        Ok(())
    }
}

