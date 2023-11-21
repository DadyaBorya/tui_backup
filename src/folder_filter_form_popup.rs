use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Clear};
use crate::app::{App};
use crate::app_mode::{AppMode, FolderListFilter};
use crate::file_item_list_filter::FolderFilter;
use crate::file_system::FileSystemItem;
use crate::popup::Popup;
use crate::widget_gen::WidgetGen;

#[derive(Debug, Clone)]
pub struct FolderFilterFormPopup {}

impl FolderFilterFormPopup {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        if let AppMode::FolderListFilter(FolderListFilter::List) = app.mode {
            return;
        }

        if let AppMode::FolderListFilter(_) = app.mode {
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
                        Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
                    ].as_ref()
                )
                .split(area);

            let regex_input = WidgetGen::form_input(
                "Regex",
                app.file_item_list_filter.new_regex.as_str(),
                match app.mode {
                    AppMode::FolderListFilter(FolderListFilter::Regex) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(regex_input, chunks[0]);

            let deep_input = WidgetGen::form_input(
                "Deep",
                app.file_item_list_filter.new_deep.as_str(),
                match app.mode {
                    AppMode::FolderListFilter(FolderListFilter::Deep) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(deep_input, chunks[1]);

            let submit_btn = WidgetGen::form_button(
                "Submit",
                match app.mode {
                    AppMode::FolderListFilter(FolderListFilter::Submit) => Style::default().fg(Color::Yellow),
                    _ => Style::default()
                }
            );

            f.render_widget(submit_btn, chunks[2]);
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FolderListFilter(FolderListFilter::Form) => {
                match key_code {
                    KeyCode::Esc => {
                        app.file_item_list_filter.clean_inputs();
                        app.change_mode(AppMode::FolderListFilter(FolderListFilter::List));
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FolderListFilter(FolderListFilter::Regex)),
                    _ => {}
                }
            }
            AppMode::FolderListFilter(FolderListFilter::Regex)=> {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListFilter(FolderListFilter::Form));
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FolderListFilter(FolderListFilter::Deep)),
                    KeyCode::Char(c) => {
                        app.file_item_list_filter.new_regex.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_filter.new_regex.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FolderListFilter(FolderListFilter::Deep) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListFilter(FolderListFilter::Form));
                    }
                    KeyCode::Tab => app.change_mode(AppMode::FolderListFilter(FolderListFilter::Submit)),
                    KeyCode::BackTab => app.change_mode(AppMode::FolderListFilter(FolderListFilter::Regex)),
                    KeyCode::Char(c) => {
                        app.file_item_list_filter.new_deep.push(c);
                    }
                    KeyCode::Backspace => {
                        app.file_item_list_filter.new_deep.pop();
                    }
                    _ => {}
                }
            }
            AppMode::FolderListFilter(FolderListFilter::Submit) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListFilter(FolderListFilter::Form));
                    }
                    KeyCode::BackTab => app.change_mode(AppMode::FolderListFilter(FolderListFilter::Deep)),
                    KeyCode::Enter => {
                        let regex = app.file_item_list_filter.new_regex.to_owned();
                        let deep = app.file_item_list_filter.new_deep.to_owned();
                        let folder_filter = FolderFilter::new(regex, deep);

                        if let Some(item) = app.file_list.get_current_item() {
                            if let FileSystemItem::Folder_(folder) = item {
                                if app.is_edit_folder_filter_form_popup {
                                    if let Some(index) = app.file_item_list_filter.folder_filter_list.selected() {
                                        folder.folder_filter_rules[index] = folder_filter;
                                        app.is_edit_folder_filter_form_popup = false;
                                    }
                                } else {
                                    folder.folder_filter_rules.push(folder_filter);
                                }
                                app.file_item_list_filter.clean_inputs();
                                app.change_mode(AppMode::FolderListFilter(FolderListFilter::List));
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