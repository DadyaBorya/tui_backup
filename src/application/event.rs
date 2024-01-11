use crate::components::dir::priority::dir::component::DirPriorityComponent;
use crate::components::dir::priority::dir::form::component::DirPriorityFormComponent;
use crate::components::dir::priority::file::component::DirFilePriorityComponent;
use crate::components::dir::priority::file::form::component::DirFilePriorityFormComponent;
use crate::components::file::filter::component::FileFilterComponent;
use crate::components::file::filter::form::component::FileFilterFormComponent;
use crate::components::file::list::component::FileListComponent;
use crate::components::file::list::settings::component::FileListSettingComponent;
use crate::components::file::priority::component::FilePriorityComponent;
use crate::components::file::priority::form::component::FilePriorityFormComponent;
use crate::components::popup::confirm::component::ConfirmPopupComponent;
use crate::components::popup::message::component::MessagePopupComponent;
use crate::components::scheduler::form::component::SchedulerFormComponent;
use crate::components::scheduler::list::component::SchedulerListComponent;
use crate::components::tab::component::TabComponent;
use crate::components::template::form::component::TemplateFormComponent;
use crate::components::template::list::component::TemplateListComponent;
use crossterm::event::Event::Key;
use crossterm::event::{self, KeyEventKind};

use super::app::App;
use super::mode::AppMode;

impl App {
    pub fn event(&mut self) -> Result<(), std::io::Error> {
        event::poll(std::time::Duration::from_millis(16))?;
        {
            if let Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.state.mode {
                        AppMode::FileList => FileListComponent::event(self, key.code)?,
                        AppMode::MessagePopup => MessagePopupComponent::event(self, key.code),
                        AppMode::FileFilter => FileFilterComponent::event(self, key.code)?,
                        AppMode::FilePriority => FilePriorityComponent::event(self, key.code)?,
                        AppMode::DirPriority => DirPriorityComponent::event(self, key.code)?,
                        AppMode::DirFilePriority => {
                            DirFilePriorityComponent::event(self, key.code)?
                        }
                        AppMode::FileFilterForm(_) => {
                            FileFilterFormComponent::event(self, key.code)?
                        }
                        AppMode::DirFilePriorityForm(_) => {
                            DirFilePriorityFormComponent::event(self, key.code)?
                        }
                        AppMode::DirPriorityForm(_) => {
                            DirPriorityFormComponent::event(self, key.code)?
                        }
                        AppMode::FilePriorityForm(_) => {
                            FilePriorityFormComponent::event(self, key.code)?
                        }
                        AppMode::SchedulerForm(_) => SchedulerFormComponent::event(self, key.code)?,
                        AppMode::SchedulerList => SchedulerListComponent::event(self, key.code)?,
                        AppMode::Tab => TabComponent::event(self, key.code)?,
                        AppMode::TemplateForm(_) => TemplateFormComponent::event(self, key.code)?,
                        AppMode::TemplateList => TemplateListComponent::event(self, key.code)?,
                        AppMode::FileListSettings => FileListSettingComponent::event(self, key.code)?,
                        AppMode::Confirm(_) => ConfirmPopupComponent::event(self, key.code)  
                    }
                }
            }
        }
        Ok(())
    }
}
