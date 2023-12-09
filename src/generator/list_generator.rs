use tui::{
    widgets::{ ListItem, List, Block, Borders, BorderType },
    layout::Alignment,
    style::{ Style, Modifier },
};

use crate::application::app::ACTIVE_BORDER_COLOR;

pub fn list(title: String, is_selected: bool, items: Vec<String>) -> List<'static> {
    let items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(item.clone()))
        .collect();

    let style = match is_selected {
        true => Style::default().fg(ACTIVE_BORDER_COLOR),
        false => Style::default(),
    };

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(style)
                .title(title)
                .title_alignment(Alignment::Center)
        )
        .highlight_symbol("->")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}
