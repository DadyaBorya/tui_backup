use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    application::{
        app::{App, ACTIVE_BORDER_COLOR},
        mode::{AppMode, Confirm},
    },
    generator::{input_generator, popup},
};

use super::component::ConfirmPopupComponent;

impl ConfirmPopupComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let state = &app.components.confirm.state;

        let block = Block::default()
            .title(state.title.as_str())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ACTIVE_BORDER_COLOR))
            .border_type(BorderType::Rounded);

        let area = popup::centered_rect(50, 25, f.size());
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let chunks = Layout::default()
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area);

        let text = Paragraph::new(state.message.as_str()).alignment(Alignment::Center);

        f.render_widget(text, chunks[0]);

        let buttons_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[2]);

        let cancel_button = input_generator::button(
            "Cancel".to_string(),
            app.state.mode == AppMode::Confirm(Confirm::Cancel),
        );
        f.render_widget(cancel_button, buttons_chunks[0]);

        let submit_button = input_generator::button(
            "Submit".to_string(),
            app.state.mode == AppMode::Confirm(Confirm::Submit),
        );
        f.render_widget(submit_button, buttons_chunks[1]);
    }
}
