use crate::{
    components::{
        file_list::file_list_component::FileListComponent,
        tab::tab_component::TabComponent,
        message_popup::message_popup_components::MessagePopupComponent,
        file_filter::file_filter_component::FileFilterComponent,
        dir_filter::dir_filter_component::DirFilterComponent,
        dir_file_priority::dir_file_priority_component::DirFilePriorityComponent,
        dir_priority::dir_priority_component::DirPriorityComponent,
        file_priority::file_priority_component::FilePriorityComponent,
        file_filter_form::file_filter_form_component::FileFilterFormComponent,
        dir_filter_form::dir_filter_form_component::DirFilterFormComponent,
        dir_file_priority_form::dir_file_priority_form_component::DirFilePriorityFormComponent,
        dir_priority_form::dir_priority_form_component::DirPriorityFormComponent,
        file_priority_form::file_priority_form_component::FilePriorityFormComponent,
        template_list::template_list_component::TemplateListComponent,
        scheduler_form::scheduler_form_component::SchedulerFormComponent,
        template_form::template_form_component::TemplateFormComponent,
        scheduler_list::scheduler_list_component::SchedulerListComponent,
    },
    models::config::Config,
};

use super::app_mode::AppMode;

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
    pub dir_filter: DirFilterComponent,
    pub dir_file_priority: DirFilePriorityComponent,
    pub dir_priority: DirPriorityComponent,
    pub file_priority: FilePriorityComponent,
    pub file_filter_form: FileFilterFormComponent,
    pub dir_filter_form: DirFilterFormComponent,
    pub dir_file_priority_form: DirFilePriorityFormComponent,
    pub dir_priority_form: DirPriorityFormComponent,
    pub file_priority_form: FilePriorityFormComponent,
    pub template_form: TemplateFormComponent,
    pub template_list: TemplateListComponent,
    pub scheduler_form: SchedulerFormComponent,
    pub scheduler_list: SchedulerListComponent,
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
            dir_filter: DirFilterComponent::init(),
            dir_file_priority: DirFilePriorityComponent::init(),
            dir_priority: DirPriorityComponent::init(),
            file_priority: FilePriorityComponent::init(),
            file_filter_form: FileFilterFormComponent::init(),
            dir_filter_form: DirFilterFormComponent::init(),
            dir_file_priority_form: DirFilePriorityFormComponent::init(),
            dir_priority_form: DirPriorityFormComponent::init(),
            file_priority_form: FilePriorityFormComponent::init(),
            template_form: TemplateFormComponent::init(),
            template_list: TemplateListComponent::init(config)?,
            scheduler_form: SchedulerFormComponent::init(),
            scheduler_list: SchedulerListComponent::init(config)?,
        })
    }
}
