use tui::{
    backend::Backend,
    Frame,
    layout::Rect,
    text::{ Spans, Span },
    style::{ Style, Color },
    widgets::{ Tabs, Block, Borders, BorderType },
};

use crate::application::app::App;

const HEADER_FG: Color = Color::White;
const SELECTED_HEADER_FG: Color = Color::Yellow;
const CODE_NAME: &'static str = "Have The Guts";

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    let tabs = &mut app.state.components.tabs;
    let headers = create_headers(&tabs.state.headers, tabs.state.index);
    let tabs = create_tabs(headers);
    f.render_widget(tabs, chunks[0]);
}

fn create_headers(headers: &Vec<String>, current_index: usize) -> Vec<Spans<'_>> {
    headers
        .iter()
        .enumerate()
        .map(|(index, h)| {
            match index == current_index {
                true =>
                    Spans::from(
                        vec![Span::styled(h.to_owned(), Style::default().fg(SELECTED_HEADER_FG))]
                    ),
                false =>
                    Spans::from(vec![Span::styled(h.to_owned(), Style::default().fg(HEADER_FG))]),
            }
        })
        .collect::<Vec<Spans<'_>>>()
}

fn create_tabs(headers: Vec<Spans<'_>>) -> Tabs<'_> {
    Tabs::new(headers).block(
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(CODE_NAME)
    )
}
