use crossterm::event::{ self, KeyEventKind };
use crossterm::event::Event::Key;
use crate::components::dir_filter::dir_filter_event;
use crate::components::file_filter::file_filter_event;
use crate::components::file_list::file_list_event;
use crate::components::message_popup::message_popup_event;
use crate::components::tab::tab_event;

use super::app::App;
use super::app_mode::AppMode;

pub fn event(app: &mut App) -> Result<(), std::io::Error> {
    event::poll(std::time::Duration::from_millis(16))?;
    {
        if let Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.state.mode {
                    AppMode::Tab => tab_event::event(app, key.code)?,
                    AppMode::FileList => file_list_event::event(app, key.code)?,
                    AppMode::MessagePopup => message_popup_event::event(app, key.code),
                    AppMode::FileFilter => file_filter_event::event(app, key.code)?,
                    AppMode::DirFilter => dir_filter_event::event(app, key.code)?,
                    AppMode::FilePriority => todo!(),
                    AppMode::DirPriority => todo!(),
                    AppMode::DirFilePriority => todo!(),
                }
            }
        }
    }
    Ok(())
}
