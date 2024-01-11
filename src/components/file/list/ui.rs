use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    application::{app::App, mode::AppMode},
    components::{
        dir::priority::{
            dir::component::DirPriorityComponent, file::component::DirFilePriorityComponent,
        },
        file::{
            filter::component::FileFilterComponent, priority::component::FilePriorityComponent,
        },
    },
    generator::table_generator,
};

use super::component::FileListComponent;

const HEADERS: [&'static str; 3] = ["", "Name", "Extension"];

impl FileListComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let file_list = &mut app.components.file_list;

        let is_file = !file_list.state.is_selected_dir();
        let items = app
            .components
            .file_list_settings
            .state
            .seleted_items
            .clone();

        let chunks = FileListComponent::get_main_chunks(
            file_list.state.is_selected(),
            chunks,
            is_file,
            items,
        );

        let table = table_generator::table(
            HEADERS.to_vec(),
            &file_list.state.table_rows,
            file_list.state.current_path.to_str().unwrap(),
            app.state.mode == AppMode::FileList,
        )
        .widths(&[
            Constraint::Length(3),
            Constraint::Length(40),
            Constraint::Min(10),
        ]);

        f.render_stateful_widget(table, chunks[0], &mut file_list.state.table_state);

        if file_list.state.is_selected() {
            FileListComponent::file_list_actions(&chunks, app, f);
        }
    }

    fn get_main_chunks(
        is_selected: bool,
        chunks: &Vec<Rect>,
        is_file: bool,
        items: Vec<usize>,
    ) -> Vec<Rect> {
        let contraints_len = FileListComponent::contraints_for_action_chunks(is_file, items).len();

        match !is_selected || contraints_len == 0 {
            true => Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(chunks[1]),
            false => Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
                .split(chunks[1]),
        }
    }

    fn file_list_actions<B: Backend>(chunks: &Vec<Rect>, app: &mut App, f: &mut Frame<B>) {
        let entry = app.components.file_list.state.get_selected_entry().unwrap();

        let constraints = FileListComponent::contraints_for_action_chunks(
            !entry.is_dir(),
            app.components
                .file_list_settings
                .state
                .seleted_items
                .clone(),
        );

        if constraints.is_empty() {
            return;
        }

        let action_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(chunks[1]);

        app.components.file_filter.state.rules =
            entry.entry_file_filter.clone().unwrap_or_default();
        app.components.dir_file_priority.state.rules =
            entry.entry_dir_file_priority.clone().unwrap_or_default();
        app.components.dir_priority.state.rules =
            entry.entry_dir_priority.clone().unwrap_or_default();
        app.components.file_priority.state.rules =
            entry.entry_file_priority.clone().unwrap_or_default();

        let mut items = app
            .components
            .file_list_settings
            .state
            .seleted_items
            .clone();

        match entry.is_dir() {
            true => {
                items.retain(|i| i != &3);

                let mut index = 0;

                if items.contains(&0) {
                    FileFilterComponent::ui(app, f, &action_chunks, index);
                    index += 1;
                }

                if items.contains(&1) {
                    DirFilePriorityComponent::ui(app, f, &action_chunks, index);
                    index += 1;
                }

                if items.contains(&2) {
                    DirPriorityComponent::ui(app, f, &action_chunks, index);
                }
            }
            false => {
                if items.contains(&3) {
                    FilePriorityComponent::ui(app, f, &action_chunks);
                }
            }
        }
    }

    fn contraints_for_action_chunks(is_file: bool, items: Vec<usize>) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        return match is_file {
            true => {
                if items.contains(&3) {
                    constraints.push(Constraint::Percentage(100));
                }
                constraints
            }
            false => {
                let offset = match items.contains(&3) {
                    true => 1,
                    false => 0,
                };

                if items.len() - offset <= 0 {
                    return vec![];
                }

                let percentage = 100 / (items.len() - offset);
                for _ in 0..items.len() - offset {
                    constraints.push(Constraint::Percentage(percentage as u16));
                }
                constraints
            }
        };
    }
}
