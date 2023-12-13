use crate::components::{
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
};

use super::app_mode::AppMode;

pub struct AppState {
    pub mode: AppMode,
    pub prev_mode: AppMode,
    pub flags: AppFlags,
    pub exit: bool,
}

#[derive(Default)]
pub struct AppFlags {
    pub is_edit_folder_filter_form_popup: bool,
    pub is_edit_file_filter_form_popup: bool,
    pub is_edit_folder_priority_form_popup: bool,
    pub is_edit_file_folder_priority_form_popup: bool,
    pub is_edit_file_priority_form_popup: bool,
    pub is_edit_template_list: bool,
}

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
    pub fn init() -> Result<Self, std::io::Error> {
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
        })
    }
}
