use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Clear};
use crate::app::App;
use crate::app_mode::{AppMode, FileListPriority};
use crate::file_item_list_priority::FilePriority;
use crate::file_system::FileSystemItem;
use crate::popup::Popup;
use crate::widget_gen::WidgetGen;

pub struct FileListPriorityFormPopup {}

impl FileListPriorityFormPopup {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        if let AppMode::FileListPriority(FileListPriority::List) = app.mode {
            return;
        }

        if let AppMode::FileListPriority(_) = app.mode {
            let block = Block::default()
                .title("File Priority")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let area = Popup::centered_rect(60, 70, f.size());
            f.render_widget(Clear, area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .margin(2)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(0), Constraint::Length(3), Constraint::Length(3)
                    ].as_ref()
                ).split(area);

            let content_input = WidgetGen::form_input(
                "Content",
                app.file_item_list_priority.new_content.as_str(),
                match app.mode {
                    AppMode::FileListPriority(FileListPriority::Content) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                },
            );

            f.render_widget(content_input, chunks[0]);

            let priority_input = WidgetGen::form_input(
                "Priority",
                app.file_item_list_priority.new_priority.as_str(),
                match app.mode {
                    AppMode::FileListPriority(FileListPriority::Priority) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                },
            );

            f.render_widget(priority_input, chunks[1]);
            let submit_btn = WidgetGen::form_button(
                "Submit",
                match app.mode {
                    AppMode::FileListPriority(FileListPriority::Submit) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                },
            );

            f.render_widget(submit_btn, chunks[2]);
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FileListPriority(FileListPriority::Form) => {
                match key_code {
                    KeyCode::Esc => {
                        app.file_item_list_priority.clean_inputs();
                        app.change_mode(AppMode::FileListPriority(FileListPriority::List));
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileListPriority(FileListPriority::Content)),
                    KeyCode::Char('h') => {
                        app.prev_mode = AppMode::FileListPriority(FileListPriority::Form);
                        app.change_mode(AppMode::HelpPopup);
                    }
                    _ => {}
                }
            }
            AppMode::FileListPriority(FileListPriority::Content) => {
                match key_code {
                    KeyCode::Esc => app.change_mode(AppMode::FileListPriority(FileListPriority::Form)),
                    KeyCode::Tab => app.change_mode(AppMode::FileListPriority(FileListPriority::Priority)),
                    KeyCode::BackTab => app.change_mode(AppMode::FileListPriority(FileListPriority::Content)),
                    KeyCode::Char(c) => {
                        app.file_item_list_priority.new_content.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_priority.new_content.pop();
                    }
                    KeyCode::Enter => {
                        app.file_item_list_priority.new_content.push('\n');
                    }
                    _ => {}
                }
            }
            AppMode::FileListPriority(FileListPriority::Priority) => {
                match key_code {
                    KeyCode::Esc => app.change_mode(AppMode::FileListPriority(FileListPriority::Form)),
                    KeyCode::Tab => app.change_mode(AppMode::FileListPriority(FileListPriority::Submit)),
                    KeyCode::BackTab => app.change_mode(AppMode::FileListPriority(FileListPriority::Content)),
                    KeyCode::Char(c) => {
                        app.file_item_list_priority.new_priority.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_priority.new_priority.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileListPriority(FileListPriority::Submit) => {
                match key_code {
                    KeyCode::Esc => app.change_mode(AppMode::FileListPriority(FileListPriority::Form)),
                    KeyCode::BackTab => app.change_mode(AppMode::FileListPriority(FileListPriority::Priority)),
                    KeyCode::Enter => {
                        let content = app.file_item_list_priority.new_content.to_owned();
                        let priority = app.file_item_list_priority.new_priority.to_owned();

                        let file_priority = FilePriority::new(content, priority);

                        if let Some(item) = app.file_list.get_current_item() {
                            if let FileSystemItem::File_(file) = item {
                                if app.is_edit_file_priority_form_popup {
                                    if let Some(index) = app.file_item_list_priority.file_priority_list.selected() {
                                        file.file_priority_rules[index] = file_priority;
                                        app.is_edit_file_priority_form_popup = false;
                                    }
                                } else {
                                    file.file_priority_rules.push(file_priority);
                                }
                                app.file_item_list_priority.clean_inputs();
                                app.change_mode(AppMode::FileListPriority(FileListPriority::List));
                            }
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