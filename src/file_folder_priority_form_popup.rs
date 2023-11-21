use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Clear};
use crate::app::App;
use crate::app_mode::{AppMode, FileFolderListPriority};
use crate::file_item_list_priority::FileFolderPriority;
use crate::file_system::FileSystemItem;
use crate::popup::Popup;
use crate::widget_gen::WidgetGen;

pub struct FileFolderPriorityFormPopup {}

impl FileFolderPriorityFormPopup {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        if let AppMode::FileFolderListPriority(FileFolderListPriority::List) = app.mode {
            return;
        }

        if let AppMode::FileFolderListPriority(_) = app.mode {
            let block = Block::default()
                .title("File Priority")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let area = Popup::centered_rect(60, 80, f.size());
            f.render_widget(Clear, area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .margin(2)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3), Constraint::Length(3), Constraint::Min(0), Constraint::Length(3), Constraint::Length(3)
                    ].as_ref()
                ).split(area);

            let regex_input = WidgetGen::form_input(
                "Regex",
                app.file_item_list_priority.new_regex.as_str(),
                match app.mode {
                    AppMode::FileFolderListPriority(FileFolderListPriority::Regex) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(regex_input, chunks[0]);

            let deep_input = WidgetGen::form_input(
                "Deep",
                app.file_item_list_priority.new_deep.as_str(),
                match app.mode {
                    AppMode::FileFolderListPriority(FileFolderListPriority::Deep) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(deep_input, chunks[1]);

            let content_input = WidgetGen::form_input(
                "Content",
                app.file_item_list_priority.new_content.as_str(),
                match app.mode {
                    AppMode::FileFolderListPriority(FileFolderListPriority::Content) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(content_input, chunks[2]);

            let priority_input = WidgetGen::form_input(
                "Priority",
                app.file_item_list_priority.new_priority.as_str(),
                match app.mode {
                    AppMode::FileFolderListPriority(FileFolderListPriority::Priority) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(priority_input, chunks[3]);

            let submit_btn = WidgetGen::form_button(
                "Submit",
                match app.mode {
                    AppMode::FileFolderListPriority(FileFolderListPriority::Submit) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(submit_btn, chunks[4]);
        }
    }
    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FileFolderListPriority(FileFolderListPriority::Form) => {
                match key_code {
                    KeyCode::Esc => {
                        app.file_item_list_priority.clean_inputs();
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::List))
                    },
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Regex))
                    }
                    _ => {}
                }
            },
            AppMode::FileFolderListPriority(FileFolderListPriority::Regex) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Form))
                    },
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Deep))
                    }
                    KeyCode::Char(c) => {
                        app.file_item_list_priority.new_regex.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_priority.new_regex.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileFolderListPriority(FileFolderListPriority::Deep) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Form))
                    },
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Content))
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Regex))
                    }
                    KeyCode::Char(c) => {
                        app.file_item_list_priority.new_deep.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_priority.new_deep.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileFolderListPriority(FileFolderListPriority::Content) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Form))
                    },
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Priority))
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Deep))
                    }
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
            AppMode::FileFolderListPriority(FileFolderListPriority::Priority) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Form))
                    },
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Submit))
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Content))
                    }
                    KeyCode::Char(c) => {
                        app.file_item_list_priority.new_priority.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_priority.new_priority.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileFolderListPriority(FileFolderListPriority::Submit) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Form))
                    },
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::Priority))
                    },
                    KeyCode::Enter => {
                        let regex = app.file_item_list_priority.new_regex.to_owned();
                        let deep = app.file_item_list_priority.new_deep.to_owned();
                        let content = app.file_item_list_priority.new_content.to_owned();
                        let priority = app.file_item_list_priority.new_priority.to_owned();

                        let file_priority = FileFolderPriority::new(regex, deep, content, priority);

                        if let Some(item) = app.file_list.get_current_item() {
                            if let FileSystemItem::Folder_(folder) = item {
                                if app.is_edit_file_folder_priority_form_popup {
                                    if let Some(index) = app.file_item_list_priority.file_folder_priority_list.selected() {
                                        folder.file_priority_rules[index] = file_priority;
                                        app.is_edit_file_folder_priority_form_popup = false;
                                    }
                                } else {
                                    folder.file_priority_rules.push(file_priority);

                                }
                                app.file_item_list_priority.clean_inputs();
                                app.change_mode(AppMode::FileFolderListPriority(FileFolderListPriority::List));
                            }
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}