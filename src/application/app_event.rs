use crossterm::event::{ self, KeyEventKind };
use crossterm::event::Event::Key;
use crate::components::scheduler_form::scheduler_form_event;
use crate::components::dir_file_priority::dir_file_priority_event;
use crate::components::dir_file_priority_form::dir_file_priority_form_event;
use crate::components::dir_filter::dir_filter_event;
use crate::components::dir_filter_form::dir_filter_form_event;
use crate::components::dir_priority::dir_priority_event;
use crate::components::dir_priority_form::dir_priority_form_event;
use crate::components::file_filter::file_filter_event;
use crate::components::file_filter_form::file_filter_form_event;
use crate::components::file_list::file_list_event;
use crate::components::file_priority::file_priority_event;
use crate::components::file_priority_form::file_priority_form_event;
use crate::components::message_popup::message_popup_event;
use crate::components::tab::tab_event;
use crate::components::template_form::template_form_event;
use crate::components::template_list::template_list_event;

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
                    AppMode::FilePriority => file_priority_event::event(app, key.code)?,
                    AppMode::DirPriority => dir_priority_event::event(app, key.code)?,
                    AppMode::DirFilePriority => dir_file_priority_event::event(app, key.code)?,
                    AppMode::FileFilterForm(_) => file_filter_form_event::event(app, key.code)?,
                    AppMode::DirFilterForm(_) => dir_filter_form_event::event(app, key.code)?,
                    AppMode::DirFilePriorityForm(_) =>
                        dir_file_priority_form_event::event(app, key.code)?,
                    AppMode::DirPriorityForm(_) => dir_priority_form_event::event(app, key.code)?,
                    AppMode::FilePriorityForm(_) => file_priority_form_event::event(app, key.code)?,
                    AppMode::TemplateForm(_) => template_form_event::event(app, key.code)?,
                    AppMode::TemplateList => template_list_event::event(app, key.code)?,
                    AppMode::SchedulerForm(_) => scheduler_form_event::event(app, key.code)?,
                }
            }
        }
    }
    Ok(())
}
