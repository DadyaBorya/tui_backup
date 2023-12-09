use tui::{
    style::{ Style, Color, Modifier },
    widgets::{ Row, Cell, Table, Block, Borders, BorderType },
    layout::Alignment,
};

use crate::application::app::ACTIVE_BORDER_COLOR;

const SELECTED_COLOR: Color = Color::Yellow;
const NORMAL_COLOR: Color = Color::White;

pub fn table<'a>(
    headers: Vec<&'static str>,
    rows: &'a Vec<(Vec<String>, Color)>,
    title: &'a str,
    is_selected: bool
) -> Table<'a> {
    let normal_style = Style::default().bg(NORMAL_COLOR);

    let header_cells = headers
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));

    let header = Row::new(header_cells).style(normal_style).height(1).bottom_margin(1);

    let rows = rows.iter().map(|item| {
        let height =
            item.0
                .iter()
                .map(|content|
                    content
                        .chars()
                        .filter(|c| *c == '\n')
                        .count()
                )
                .max()
                .unwrap_or(0) + 1;
        let cells = item.0.iter().map(|c| Cell::from(c.as_str()));
        Row::new(cells)
            .style(Style::default().fg(item.1))
            .height(height as u16)
            .bottom_margin(1)
    });

    let selected_style = Style::default().add_modifier(Modifier::REVERSED).fg(SELECTED_COLOR);

    let style = match is_selected {
        true => Style::default().fg(ACTIVE_BORDER_COLOR),
        false => Style::default(),
    };

    Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title_alignment(Alignment::Center)
                .title(title)
                .border_style(style)
        )
        .highlight_style(selected_style)
        .highlight_symbol("->")
}
