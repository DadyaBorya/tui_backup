use crossterm::event::{ self, KeyEventKind };
use crossterm::event::Event::Key;
use crate::tab_component::tab_event;

use super::app::App;
use super::app_mode::AppMode;

pub fn event(app: &mut App) -> Result<(), std::io::Error> {
    event::poll(std::time::Duration::from_millis(16))?;
    {
        if let Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.state.mode {
                    AppMode::Tab => tab_event::event(app, key.code)?,
                    AppMode::FileList => todo!(),
                    AppMode::ErrorPopup => todo!(),
                    AppMode::HelpPopup => todo!(),
                    AppMode::TemplateList => todo!(),
                    AppMode::SchedulerList => todo!(),
                    AppMode::FolderListFilter(_) => todo!(),
                    AppMode::FileFolderListFilter(_) => todo!(),
                    AppMode::FolderListPriority(_) => todo!(),
                    AppMode::FileFolderListPriority(_) => todo!(),
                    AppMode::FileListPriority(_) => todo!(),
                    AppMode::CreateTemplate(_) => todo!(),
                    AppMode::CreateScheduler(_) => todo!(),
                }
            }
        }
    }
    Ok(())
}
