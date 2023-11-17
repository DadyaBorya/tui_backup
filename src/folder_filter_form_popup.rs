use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Clear, Paragraph};
use crate::app::{App, AppMode};
use crate::file_list_filter::FolderFilter;
use crate::file_system::FileSystemItem;
use crate::popup::Popup;

#[derive(Debug, Clone)]
pub struct FolderFilterFormPopup {}

impl FolderFilterFormPopup {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        if app.is_folder_filter_form_popup {
            let block = Block::default()
                .title("Folder Filter")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let area = Popup::centered_rect(60, 40, f.size());
            f.render_widget(Clear, area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ].as_ref()
                )
                .split(area);

            let regex_input = Paragraph::new(app.file_list_filter.new_regex.to_owned())
                .block(Block::default().title("Regex").borders(Borders::ALL).border_type(BorderType::Rounded))
                .style(match app.mode {
                    AppMode::FolderListFilterFormRegex => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                });

            f.render_widget(regex_input, chunks[0]);

            let deep_input = Paragraph::new(app.file_list_filter.new_deep.to_owned())
                .block(Block::default().title("Deep").borders(Borders::ALL).border_type(BorderType::Rounded))
                .style(match app.mode {
                    AppMode::FolderListFilterFormDeep => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                });

            f.render_widget(deep_input, chunks[1]);

            let submit_btn = Paragraph::new("Submit")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
                .style(match app.mode {
                    AppMode::FolderListFilterFormSubmit => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                });
            f.render_widget(submit_btn, chunks[2]);
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FolderListFilterForm => {
                match key_code {
                    KeyCode::Esc => {
                        app.is_folder_filter_form_popup = false;
                        app.file_list_filter.new_deep = "".to_string();
                        app.file_list_filter.new_regex = "".to_string();
                        app.change_mode(AppMode::FolderListFilter);
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FolderListFilterFormRegex),

                    _ => {}
                }
            }
            AppMode::FolderListFilterFormRegex => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListFilterForm);
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FolderListFilterFormDeep),
                    KeyCode::Char(c) => {
                        app.file_list_filter.new_regex.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_list_filter.new_regex.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FolderListFilterFormDeep => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListFilterForm);
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FolderListFilterFormSubmit),
                    KeyCode::BackTab => app.change_mode(AppMode::FolderListFilterFormRegex),
                    KeyCode::Char(c) => {
                        app.file_list_filter.new_deep.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_list_filter.new_deep.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FolderListFilterFormSubmit => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListFilterForm);
                    }
                    KeyCode::BackTab => app.change_mode(AppMode::FolderListFilterFormDeep),
                    KeyCode::Enter => {
                        let regex = app.file_list_filter.new_regex.to_owned();
                        let deep = app.file_list_filter.new_deep.to_owned();
                        let folder_filter = FolderFilter::new(regex, deep);

                        if let Some(item) = app.file_list.get_current_item() {
                            match item {
                                FileSystemItem::File_(_) => {}
                                FileSystemItem::Folder_(folder) => {
                                    if app.is_edit_folder_filter_form_popup {
                                        if let Some(index) = app.file_list_filter.folder_filter_list.selected() {
                                            folder.folder_filter_rules[index] = folder_filter;
                                            app.is_edit_folder_filter_form_popup = false;
                                        }
                                    } else {
                                        folder.folder_filter_rules.push(folder_filter);
                                    }
                                    app.is_folder_filter_form_popup = false;
                                    app.file_list_filter.new_deep = "".to_string();
                                    app.file_list_filter.new_regex = "".to_string();
                                    app.change_mode(AppMode::FolderListFilter);
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