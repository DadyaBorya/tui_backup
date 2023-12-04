use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{ Alignment, Constraint, Direction, Layout };
use tui::style::{ Color, Style };
use tui::widgets::{ Block, Borders, BorderType, Clear };
use crate::app::App;
use crate::app_mode::{ AppMode, FolderListPriority };
use crate::file_item_list_priority::FolderPriority;
use crate::file_system::FileSystemItem;
use crate::popup::Popup;
use crate::widget_gen::WidgetGen;

#[derive(Clone)]
pub struct FolderPriorityFormPopup {}

impl FolderPriorityFormPopup {
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        if let AppMode::FolderListPriority(FolderListPriority::List) = app.mode {
            return;
        }

        if let AppMode::FolderListPriority(_) = app.mode {
            let block = Block::default()
                .title("Folder Priority")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let area = Popup::centered_rect(60, 50, f.size());
            f.render_widget(Clear, area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .margin(2)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ].as_ref()
                )
                .split(area);

            let regex_input = WidgetGen::form_input(
                "Regex",
                app.file_item_list_priority.new_regex.as_str(),
                match app.mode {
                    AppMode::FolderListPriority(FolderListPriority::Regex) =>
                        Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                }
            );

            f.render_widget(regex_input, chunks[0]);

            let deep_input = WidgetGen::form_input(
                "Deep",
                app.file_item_list_priority.new_deep.as_str(),
                match app.mode {
                    AppMode::FolderListPriority(FolderListPriority::Deep) =>
                        Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                }
            );

            f.render_widget(deep_input, chunks[1]);

            let priority_input = WidgetGen::form_input(
                "Priority",
                app.file_item_list_priority.new_priority.as_str(),
                match app.mode {
                    AppMode::FolderListPriority(FolderListPriority::Priority) =>
                        Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                }
            );

            f.render_widget(priority_input, chunks[2]);

            let submit_btn = WidgetGen::form_button("Submit", match app.mode {
                AppMode::FolderListPriority(FolderListPriority::Submit) =>
                    Style::default().fg(Color::Yellow),
                _ => Style::default(),
            });

            f.render_widget(submit_btn, chunks[3]);
        }
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::FolderListPriority(FolderListPriority::Form) => {
                match key_code {
                    KeyCode::Esc => {
                        app.file_item_list_priority.clean_inputs();
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::List));
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Regex));
                    }
                    KeyCode::Char('h') => {
                        app.prev_mode = AppMode::FolderListPriority(FolderListPriority::Form);
                        app.change_mode(AppMode::HelpPopup);
                    }
                    _ => {}
                }
            }
            AppMode::FolderListPriority(FolderListPriority::Regex) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Form));
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Deep));
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
            AppMode::FolderListPriority(FolderListPriority::Deep) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Form));
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Priority));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Regex));
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
            AppMode::FolderListPriority(FolderListPriority::Priority) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Form));
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Submit));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Deep));
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
            AppMode::FolderListPriority(FolderListPriority::Submit) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Form));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::FolderListPriority(FolderListPriority::Priority));
                    }
                    KeyCode::Enter => {
                        let regex = app.file_item_list_priority.new_regex.to_owned();
                        let deep = app.file_item_list_priority.new_deep.to_owned();
                        let priority = app.file_item_list_priority.new_priority.to_owned();

                        let folder_priority = FolderPriority::new(regex, deep, priority);

                        if let Some(item) = app.file_list.get_current_item() {
                            if let FileSystemItem::Folder_(folder) = item {
                                if app.is_edit_folder_priority_form_popup {
                                    if
                                        let Some(index) =
                                            app.file_item_list_priority.folder_priority_list.selected()
                                    {
                                        let old_priority =
                                            folder.folder_priority_rules[index].clone();
                                        folder.edit_priority_by_folder(
                                            folder_priority,
                                            old_priority
                                        );
                                        app.is_edit_folder_priority_form_popup = false;
                                    }
                                } else {
                                    folder.set_up_priority_by_folder(folder_priority);
                                }

                                app.file_item_list_priority.clean_inputs();
                                app.change_mode(
                                    AppMode::FolderListPriority(FolderListPriority::List)
                                );
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
