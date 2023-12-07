use crossterm::event::KeyCode;
use rayon::iter::{ IntoParallelRefIterator, ParallelIterator };
use tui::{
    widgets::{ TableState, Cell, Row, Table, Block, Borders, BorderType },
    backend::Backend,
    Frame,
    layout::{ Rect, Layout, Direction, Constraint, Alignment },
    style::{ Style, Modifier, Color },
};

use crate::{
    app::App,
    app_mode::AppMode,
    scheduler::{ Scheduler, Cloud, Protocol },
    file_service,
    file_system::FileSystemItem,
};

#[derive(Clone)]
pub struct SchedulerList {
    pub schedulers: Vec<Scheduler>,
    pub table: TableState,
    pub rows: Vec<Vec<String>>,
}

impl SchedulerList {
    pub fn new() -> Self {
        let mut scheduler_list = SchedulerList {
            table: TableState::default(),
            schedulers: vec![],
            rows: vec![],
        };
        scheduler_list.renew_schedules();
        scheduler_list.set_rows();
        scheduler_list
    }

    pub fn renew_schedules(&mut self) {
        let items = file_service::get_system_items_from_path("schedulers").unwrap_or_default();

        self.schedulers = items
            .par_iter()
            .filter_map(|item| {
                if let FileSystemItem::File_(file) = item {
                    if let Ok(content) = file_service::get_file_content(&file.path) {
                        if let Ok(scheduler) = serde_json::from_str::<Scheduler>(&content) {
                            return Some(scheduler);
                        }
                    }
                }
                None
            })
            .collect::<Vec<Scheduler>>();
    }

    pub fn set_rows(&mut self) {
        let rows = self.schedulers
            .iter()
            .map(|s| {
                vec![
                    s.name.to_owned(),
                    Cloud::vec_to_string(&s.clouds),
                    Protocol::vec_to_string(&s.protocols),
                    s.cron.to_owned(),
                    s.speed_limit.to_string()
                ]
            })
            .collect::<Vec<Vec<String>>>();

        self.rows = rows;
    }

    pub fn init_index_table(&mut self) {
        let selected = self.table.selected();
        let length = self.rows.len();

        if length > 0 {
            if let None = selected {
                self.set_index_table(Some(0));
            }
        }
    }

    pub fn set_index_table(&mut self, index: Option<usize>) {
        self.table.select(index);
    }

    pub fn next(&mut self) {
        if self.rows.is_empty() {
            self.table.select(None);
        }

        let i = match self.table.selected() {
            Some(i) => {
                if i >= self.rows.len() - 1 { Some(i) } else { Some(i + 1) }
            }
            None => None,
        };
        self.table.select(i);
    }
    pub fn previous(&mut self) {
        if self.rows.is_empty() {
            self.table.select(None);
        }

        let i = match self.table.selected() {
            Some(i) => {
                if i == 0 { Some(i) } else { Some(i - 1) }
            }
            None => None,
        };
        self.table.select(i);
    }

    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let list_chunks = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(chunks[1]);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow);
        let normal_style = Style::default().bg(Color::White);

        let header_cells = ["Name", "Clouds", "Protocols", "Cron", "Speed limit"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));

        let header = Row::new(header_cells).style(normal_style).height(1).bottom_margin(1);

        let rows = app.scheduler_list.rows.iter().map(|item| {
            let height =
                item
                    .iter()
                    .map(|content|
                        content
                            .chars()
                            .filter(|c| *c == '\n')
                            .count()
                    )
                    .max()
                    .unwrap_or(0) + 1;
            let cells = item.iter().map(|c| Cell::from(c.as_str()));
            Row::new(cells)
                .height(height as u16)
                .bottom_margin(1)
        });

        let t = Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title_alignment(Alignment::Center)
                    .title("Scheduler List")
            )
            .highlight_style(selected_style)
            .highlight_symbol("->")
            .widths(
                &[
                    Constraint::Min(20),
                    Constraint::Min(20),
                    Constraint::Min(20),
                    Constraint::Min(20),
                    Constraint::Min(20),
                ]
            );
        f.render_stateful_widget(t, list_chunks[0], &mut app.scheduler_list.table);
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Esc => {
                app.scheduler_list.set_index_table(None);
                app.change_mode(AppMode::Tab);
            }
            KeyCode::Up => {
                app.scheduler_list.previous();
            }
            KeyCode::Down => {
                app.scheduler_list.next();
            }
            _ => {}
        }
        Ok(())
    }
}
