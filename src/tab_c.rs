use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Tabs};
use crate::app::{App, AppMode};

pub struct TabC<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabC<'a> {
    pub fn new() -> Self {
        TabC {
            titles: vec!["Tree", "Schemes"],
            index: 0,
        }
    }
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Char('q') => {
                app.exit = true;
            }
            KeyCode::Right => {
                app.tabs.next();
            }
            KeyCode::Left => {
                app.tabs.previous();
            }
            KeyCode::Char(' ') => {
                match app.tabs.index {
                    0 => {
                        app.file_list.init_index_table();
                        app.change_mode(AppMode::FileList)
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let titles = app.tabs
            .titles
            .iter()
            .map(|t| {
                Spans::from(vec![
                    Span::styled(t.to_owned(), Style::default().fg(Color::White)),
                ])
            })
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("AYLO"))
            .select(app.tabs.index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::LightCyan),
            );
        f.render_widget(tabs, chunks[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_tab() {
        let mut tabs = TabC::new();

        tabs.next();
        assert_eq!(tabs.index, 1);

        tabs.next();
        assert_eq!(tabs.index, 0);
    }

    #[test]
    fn test_previous_tab() {
        let mut tabs = TabC::new();

        tabs.previous();
        assert_eq!(tabs.index, 1);

        tabs.previous();
        assert_eq!(tabs.index, 0);

        tabs.previous();
        assert_eq!(tabs.index, 1);
    }
}
