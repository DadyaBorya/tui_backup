use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Clear, Paragraph};
use crate::app::{App, AppMode};
use crate::file_list_filter::FileFilter;
use crate::file_system::FileSystemItem;
use crate::popup::Popup;

#[derive(Debug, Clone)]
pub struct FileFilterFormPopup {}

impl FileFilterFormPopup {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        if app.is_file_filter_form_popup {
            let block = Block::default()
                .title("Folder Filter")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let area = Popup::centered_rect(60, 65, f.size());
            f.render_widget(Clear, area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(8),
                        Constraint::Length(3),
                    ].as_ref()
                )
                .split(area);

            let regex_input = Paragraph::new(app.file_list_filter.new_regex.to_owned())
                .block(Block::default().title("Regex").borders(Borders::ALL).border_type(BorderType::Rounded))
                .style(match app.mode {
                    AppMode::FileListFilterFormRegex => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                });

            f.render_widget(regex_input, chunks[0]);

            let deep_input = Paragraph::new(app.file_list_filter.new_deep.to_owned())
                .block(Block::default().title("Deep").borders(Borders::ALL).border_type(BorderType::Rounded))
                .style(match app.mode {
                    AppMode::FileListFilterFormDeep => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                });

            f.render_widget(deep_input, chunks[1]);

            let content_input = Paragraph::new(app.file_list_filter.new_content.to_owned())
                .block(Block::default().title("Content").borders(Borders::ALL).border_type(BorderType::Rounded))
                .style(match app.mode {
                    AppMode::FileListFilterFormContent => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                });

            f.render_widget(content_input, chunks[2]);

            let submit_btn = Paragraph::new("Submit")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
                .style(match app.mode {
                    AppMode::FileListFilterFormSubmit => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                });
            f.render_widget(submit_btn, chunks[3]);
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FileListFilterForm => {
                match key_code {
                    KeyCode::Esc => {
                        app.is_file_filter_form_popup = false;
                        app.file_list_filter.new_deep = "".to_string();
                        app.file_list_filter.new_regex = "".to_string();
                        app.file_list_filter.new_content = "".to_string();
                        app.change_mode(AppMode::FileListFilter);
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileListFilterFormRegex),
                    _ => {}
                }
            }
            AppMode::FileListFilterFormRegex => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileListFilterForm);
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileListFilterFormDeep),
                    KeyCode::Char(c) => {
                        app.file_list_filter.new_regex.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_list_filter.new_regex.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileListFilterFormDeep => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileListFilterForm);
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileListFilterFormContent),
                    KeyCode::BackTab => app.change_mode(AppMode::FileListFilterFormRegex),
                    KeyCode::Char(c) => {
                        app.file_list_filter.new_deep.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_list_filter.new_deep.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileListFilterFormContent => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileListFilterForm);
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileListFilterFormSubmit),
                    KeyCode::BackTab => app.change_mode(AppMode::FileListFilterFormDeep),
                    KeyCode::Enter => {
                        app.file_list_filter.new_content.push('\n');
                    }
                    KeyCode::Char(c) => {
                        app.file_list_filter.new_content.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_list_filter.new_content.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileListFilterFormSubmit => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileListFilterForm);
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FileListFilterFormContent);
                    }
                    KeyCode::Enter => {
                        let regex = app.file_list_filter.new_regex.to_owned();
                        let deep = app.file_list_filter.new_deep.to_owned();
                        let content = app.file_list_filter.new_content.to_owned();

                        let file_filter = FileFilter::new(regex, content, deep);

                        if let Some(item) = app.file_list.get_current_item() {
                            match item {
                                FileSystemItem::File_(_) => {}
                                FileSystemItem::Folder_(folder) => {
                                    if app.is_edit_file_filter_form_popup {
                                        if let Some(index) = app.file_list_filter.file_filter_list.selected() {
                                            folder.file_filter_rules[index] = file_filter;
                                            app.is_edit_file_filter_form_popup = false;
                                        }
                                    } else {
                                        folder.file_filter_rules.push(file_filter);
                                    }
                                    app.is_file_filter_form_popup = false;
                                    app.file_list_filter.new_deep = "".to_string();
                                    app.file_list_filter.new_regex = "".to_string();
                                    app.file_list_filter.new_content = "".to_string();
                                    app.change_mode(AppMode::FileListFilter);
                                }
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