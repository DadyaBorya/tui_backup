use crate::{
    components::{
        dir::priority::{
            dir::{component::DirPriorityComponent, form::component::DirPriorityFormComponent},
            file::{
                component::DirFilePriorityComponent, form::component::DirFilePriorityFormComponent,
            },
        },
        file::{
            filter::{component::FileFilterComponent, form::component::FileFilterFormComponent},
            list::{component::FileListComponent, settings::component::FileListSettingComponent},
            priority::{
                component::FilePriorityComponent, form::component::FilePriorityFormComponent,
            },
        },
        popup::{message::component::MessagePopupComponent, confirm::component::ConfirmPopupComponent},
        scheduler::{
            form::component::SchedulerFormComponent, list::component::SchedulerListComponent,
        },
        tab::component::TabComponent,
        template::{
            form::component::TemplateFormComponent, list::component::TemplateListComponent,
        },
    },
    models::config::Config,
};

use super::mode::AppMode;

pub struct AppState {
    pub mode: AppMode,
    pub prev_mode: AppMode,
    pub flags: AppFlags,
    pub exit: bool,
}

#[derive(Default)]
pub struct AppFlags {}

pub struct AppComponents {
    pub tabs: TabComponent,
    pub file_list: FileListComponent,
    pub message_popup: MessagePopupComponent,
    pub file_filter: FileFilterComponent,
    pub dir_file_priority: DirFilePriorityComponent,
    pub dir_priority: DirPriorityComponent,
    pub file_priority: FilePriorityComponent,
    pub file_filter_form: FileFilterFormComponent,
    pub dir_file_priority_form: DirFilePriorityFormComponent,
    pub dir_priority_form: DirPriorityFormComponent,
    pub file_priority_form: FilePriorityFormComponent,
    pub template_form: TemplateFormComponent,
    pub template_list: TemplateListComponent,
    pub scheduler_form: SchedulerFormComponent,
    pub scheduler_list: SchedulerListComponent,
    pub file_list_settings: FileListSettingComponent,
    pub confirm: ConfirmPopupComponent
}

impl AppState {
    pub fn init() -> Result<Self, std::io::Error> {
        Ok(AppState {
            mode: AppMode::Tab,
            prev_mode: AppMode::Tab,
            flags: AppFlags::default(),
            exit: false,
        })
    }
}

impl AppComponents {
    pub fn init(config: &Config) -> Result<Self, std::io::Error> {
        Ok(AppComponents {
            tabs: TabComponent::init(),
            file_list: FileListComponent::init()?,
            message_popup: MessagePopupComponent::init(),
            file_filter: FileFilterComponent::init(),
            dir_file_priority: DirFilePriorityComponent::init(),
            dir_priority: DirPriorityComponent::init(),
            file_priority: FilePriorityComponent::init(),
            file_filter_form: FileFilterFormComponent::init(),
            dir_file_priority_form: DirFilePriorityFormComponent::init(),
            dir_priority_form: DirPriorityFormComponent::init(),
            file_priority_form: FilePriorityFormComponent::init(),
            template_form: TemplateFormComponent::init(),
            template_list: TemplateListComponent::init(config)?,
            scheduler_form: SchedulerFormComponent::init(),
            scheduler_list: SchedulerListComponent::init(config)?,
            file_list_settings: FileListSettingComponent::init(),
            confirm: ConfirmPopupComponent::init(),
        })
    }
}
