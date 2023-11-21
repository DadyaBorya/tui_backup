use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Clear};
use crate::app::{App};
use crate::app_mode::{AppMode, FileFolderListFilter};
use crate::file_item_list_filter::FileFolderFilter;
use crate::file_system::FileSystemItem;
use crate::popup::Popup;
use crate::widget_gen::WidgetGen;

#[derive(Debug, Clone)]
pub struct FileFilterFormPopup {}

impl FileFilterFormPopup {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        if let AppMode::FileFolderListFilter(FileFolderListFilter::List) = app.mode {
            return;
        }

        if let AppMode::FileFolderListFilter(_) = app.mode {
            let block = Block::default()
                .title("Folder Filter")
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
                        Constraint::Length(3), Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)
                    ].as_ref()
                ).split(area);

            let regex_input = WidgetGen::form_input(
                "Regex",
                app.file_item_list_filter.new_regex.as_str(),
                match app.mode {
                    AppMode::FileFolderListFilter(FileFolderListFilter::Regex) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(regex_input, chunks[0]);

            let deep_input = WidgetGen::form_input(
                "Deep",
                app.file_item_list_filter.new_deep.as_str(),
                match app.mode {
                    AppMode::FileFolderListFilter(FileFolderListFilter::Deep) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(deep_input, chunks[1]);

            let content_input = WidgetGen::form_input(
                "Content",
                app.file_item_list_filter.new_content.as_str(),
                match app.mode {
                    AppMode::FileFolderListFilter(FileFolderListFilter::Content) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(content_input, chunks[2]);

            let submit_btn = WidgetGen::form_button(
                "Submit",
                match app.mode {
                    AppMode::FileFolderListFilter(FileFolderListFilter::Submit) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(submit_btn, chunks[3]);
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FileFolderListFilter(FileFolderListFilter::Form) => {
                match key_code {
                    KeyCode::Esc => {
                        app.file_item_list_filter.clean_inputs();
                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::List));
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Regex)),
                    _ => {}
                }
            }
            AppMode::FileFolderListFilter(FileFolderListFilter::Regex) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Form));
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Deep)),
                    KeyCode::Char(c) => {
                        app.file_item_list_filter.new_regex.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_filter.new_regex.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileFolderListFilter(FileFolderListFilter::Deep) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Form));
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Content)),
                    KeyCode::BackTab => app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Regex)),
                    KeyCode::Char(c) => {
                        app.file_item_list_filter.new_deep.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_filter.new_deep.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileFolderListFilter(FileFolderListFilter::Content) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Form));
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Submit)),
                    KeyCode::BackTab => app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Deep)),
                    KeyCode::Enter => {
                        app.file_item_list_filter.new_content.push('\n');
                    }
                    KeyCode::Char(c) => {
                        app.file_item_list_filter.new_content.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_filter.new_content.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FileFolderListFilter(FileFolderListFilter::Submit) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Form));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::Content));
                    }
                    KeyCode::Enter => {
                        let regex = app.file_item_list_filter.new_regex.to_owned();
                        let deep = app.file_item_list_filter.new_deep.to_owned();
                        let content = app.file_item_list_filter.new_content.to_owned();

                        let file_filter = FileFolderFilter::new(regex, content, deep);

                        if let Some(item) = app.file_list.get_current_item() {
                            if let FileSystemItem::Folder_(folder) = item {
                                if app.is_edit_file_filter_form_popup {
                                    if let Some(index) = app.file_item_list_filter.file_filter_list.selected() {
                                        folder.file_filter_rules[index] = file_filter;
                                        app.is_edit_file_filter_form_popup = false;
                                    }
                                } else {
                                    folder.file_filter_rules.push(file_filter);
                                }
                                app.file_item_list_filter.clean_inputs();
                                app.change_mode(AppMode::FileFolderListFilter(FileFolderListFilter::List));
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