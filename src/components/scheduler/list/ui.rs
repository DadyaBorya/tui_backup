use tui::{ backend::Backend, Frame, layout::{ Rect, Layout, Direction, Constraint } };

use crate::{ application::{ app::App, mode::AppMode }, generator::table_generator };

use super::component::SchedulerListComponent;

const HEADERS: [&'static str; 5] = ["Name", "Speed", "Cron", "Clouds", "Root"];

impl SchedulerListComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let scheduler_list = &mut app.components.scheduler_list;
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(chunks[1]);
    
        let rows = scheduler_list.state.rows();
    
        let table = table_generator
            ::table(HEADERS.to_vec(), &rows, "Scheduler List", app.state.mode == AppMode::SchedulerList)
            .widths(
                &[
                    Constraint::Min(20),
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Min(50),
                    Constraint::Min(20),
                ]
            );
    
        f.render_stateful_widget(table, chunks[0], &mut scheduler_list.state.table_state);
    }
}
