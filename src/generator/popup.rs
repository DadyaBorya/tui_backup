use tui::{
    layout::{ Rect, Layout, Direction, Constraint, Alignment },
    backend::Backend,
    Frame,
    widgets::{ Block, Borders, BorderType, Clear },
    style::Style,
};

use crate::application::app::ACTIVE_BORDER_COLOR;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ].as_ref()
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ].as_ref()
        )
        .split(popup_layout[1])[1]
}

pub fn popup<B: Backend>(percent_x: u16, percent_y: u16, f: &mut Frame<B>) -> Rect {
    let block = Block::default()
        .title("Create file filter")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACTIVE_BORDER_COLOR))
        .border_type(BorderType::Rounded);

    let area = centered_rect(percent_x, percent_y, f.size());
    f.render_widget(Clear, area);
    f.render_widget(block, area);
    return area;
}
