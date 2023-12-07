use crossterm::event::KeyCode;
use rayon::prelude::*;
use tui::{
    backend::Backend,
    Frame,
    layout::{ Rect, Layout, Direction, Constraint, Alignment },
    widgets::{ ListItem, List, Block, ListState, Borders, BorderType },
    text::Span,
    style::{ Style, Modifier, Color },
};

use crate::{
    file_service,
    file_system::{ FileSystemItem, Folder },
    app::App,
    app_mode::{ AppMode, CreateScheduler },
    file_list::FileList,
};

#[derive(Clone)]
pub struct TemplateList {
    pub templates: Vec<String>,
    pub list_state: ListState,
}

impl TemplateList {
    pub fn new() -> Self {
        let mut template_list = TemplateList {
            templates: vec![],
            list_state: ListState::default(),
        };
        template_list.renew_templates();
        template_list
    }

    pub fn renew_templates(&mut self) {
        let items = file_service::get_system_items_from_path("templates").unwrap_or_default();

        let templates: Vec<_> = items
            .par_iter()
            .filter_map(|item| {
                if let FileSystemItem::File_(file) = item {
                    if let Ok(content) = file_service::get_file_content(&file.path) {
                        if let Ok(_) = serde_json::from_str::<Folder>(&content) {
                            return Some(file.name.clone().replace(".json", ""));
                        }
                    }
                }
                None
            })
            .collect();

        self.templates = templates;
    }

    pub fn init_index_table(&mut self) {
        let selected = self.list_state.selected();
        let length = self.templates.len();

        if length > 0 {
            if let None = selected {
                self.set_index_table(Some(0));
            }
        }
    }
    pub fn set_index_table(&mut self, index: Option<usize>) {
        self.list_state.select(index);
    }

    pub fn next(&mut self) {
        if self.templates.is_empty() {
            self.list_state.select(None);
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.templates.len() - 1 { Some(i) } else { Some(i + 1) }
            }
            None => None,
        };
        self.list_state.select(i);
    }
    pub fn previous(&mut self) {
        if self.templates.is_empty() {
            self.list_state.select(None);
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 { Some(i) } else { Some(i - 1) }
            }
            None => None,
        };
        self.list_state.select(i);
    }

    pub fn remove_current_template(&mut self) {
        if self.templates.is_empty() || self.list_state.selected().is_none() {
            return;
        }

        let index = self.list_state.selected().unwrap();

        if index >= self.templates.len() {
            return;
        }

        if file_service::remove_file(&format!("templates/{}.json", self.templates[index])).is_err() {
            return;
        }

        self.renew_templates();

        if self.templates.is_empty() {
            self.set_index_table(None);
            return;
        }

        if index == 0 {
            self.set_index_table(Some(0));
            return;
        }

        if index - 1 > self.templates.len() {
            self.set_index_table(Some(self.templates.len() - 1));
            return;
        }

        self.set_index_table(Some(index - 1));
    }

    pub fn edit_current_template(app: &mut App) -> Result<(), std::io::Error> {
        if let Some(index) = app.template_list.list_state.selected() {
            if index < app.template_list.templates.len() {
                let template_name = app.template_list.templates[index].clone();
                let template_path = format!("templates/{}.json", &template_name);

                let template_json = file_service::get_file_content(&template_path)?;
                let template: Folder = serde_json::from_str(&template_json)?;

                app.file_list.root.reset(template);
                app.is_edit_template_list = true;
                app.create_template.form_name = template_name;
                app.tabs.index = 0;
                app.file_list.init_index_table();
                app.template_list.list_state.select(None);
                app.file_list.root.set_rows_of_current_dir();
                app.change_mode(AppMode::FileList);
            }
        }

        Ok(())
    }

    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow);

        let list_chunks = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(chunks[1]);

        let items: Vec<ListItem> = app.template_list.templates
            .clone()
            .into_iter()
            .map(|item| { ListItem::new(Span::from(item)) })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title_alignment(Alignment::Center)
                    .title("Template List")
            )
            .highlight_symbol("->")
            .highlight_style(selected_style);
        f.render_stateful_widget(list, list_chunks[0], &mut app.template_list.list_state);
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match key_code {
            KeyCode::Esc => {
                app.template_list.list_state.select(None);
                app.change_mode(AppMode::Tab);
            }
            KeyCode::Up => {
                app.template_list.previous();
            }
            KeyCode::Down => {
                app.template_list.next();
            }
            KeyCode::Char('d') => {
                app.template_list.remove_current_template();
            }
            KeyCode::Char('e') => {
                TemplateList::edit_current_template(app)?;
            }
            KeyCode::Char('n') => {
                app.create_template.clear_inputs();
                app.file_list = FileList::new()?;
                app.file_list.root.reset(app.file_list.root.root_dir.clone());
                app.is_edit_template_list = true;
                app.tabs.index = 0;
                app.file_list.init_index_table();
                app.template_list.list_state.select(None);
                app.file_list.root.set_rows_of_current_dir();
                app.change_mode(AppMode::FileList);
            }
            KeyCode::Char('h') => {
                app.prev_mode = AppMode::TemplateList;
                app.change_mode(AppMode::HelpPopup);
            }
            KeyCode::Char('c') => {
                if let Some(_) = app.template_list.list_state.selected() {
                    app.change_mode(AppMode::CreateScheduler(CreateScheduler::Form));
                }
            }
            _ => {}
        }

        Ok(())
    }
}
