use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    Frame,
    widgets::{ Block, Borders, BorderType, Clear },
    layout::{ Alignment, Layout, Direction, Constraint },
    style::{ Style, Color },
};

use crate::{
    app::App,
    popup::Popup,
    widget_gen::WidgetGen,
    app_mode::{ CreateTemplate, AppMode },
    file_service,
};

#[derive(Clone)]
pub struct CreateTemplatePopup {}

impl CreateTemplatePopup {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let block = Block::default()
            .title("Create Template")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let area = Popup::centered_rect(60, 32, f.size());
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let chunks = Layout::default()
            .margin(2)
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
            .split(area);

        let name_input = WidgetGen::form_input("Name", app.template.form_name.as_str(), match
            app.mode
        {
            AppMode::CreateTemplate(CreateTemplate::Name) => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

        f.render_widget(name_input, chunks[0]);

        let submit_btn = WidgetGen::form_button("Submit", match app.mode {
            AppMode::CreateTemplate(CreateTemplate::Submit) => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

        f.render_widget(submit_btn, chunks[1])
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::CreateTemplate(CreateTemplate::Form) => {
                match key_code {
                    KeyCode::Esc => {
                        app.template.clear_inputs();
                        app.change_mode(AppMode::FileList);
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::CreateTemplate(CreateTemplate::Name));
                    }
                    _ => {}
                }
            }
            AppMode::CreateTemplate(CreateTemplate::Name) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::CreateTemplate(CreateTemplate::Form));
                    }
                    KeyCode::Char(c) => {
                        app.template.form_name.push(c);
                    }
                    KeyCode::Backspace => {
                        app.template.form_name.pop();
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::CreateTemplate(CreateTemplate::Submit));
                    }
                    _ => {}
                }
            }
            AppMode::CreateTemplate(CreateTemplate::Submit) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::CreateTemplate(CreateTemplate::Form));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::CreateTemplate(CreateTemplate::Name));
                    }
                    KeyCode::Enter => {
                        if !app.template.form_name.is_empty() {
                            let path = format!("{}.json", app.template.form_name.as_str());

                            file_service::save_template(&app.file_list.root.root_dir, &path)?;
                            app.template.clear_inputs();
                            app.change_mode(AppMode::FileList);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}
