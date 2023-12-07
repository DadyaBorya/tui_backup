use crate::{
    file_list::FileList,
    scheduler_list::SchedulerList,
    file_item_list_filter::FileItemListFilter,
    file_item_list_priority::FileItemListPriority,
    template_list::TemplateList,
    create_template::CreateTemplate,
    create_scheduler::CreateScheduler,
    tab_component::tab_component::TabComponent,
};

use super::app_mode::AppMode;

pub struct AppState {
    pub mode: AppMode,
    pub prev_mode: AppMode,
    pub flags: AppFlags,
    pub components: AppComponents,
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
    pub file_list: FileList,
    pub scheduler_list: SchedulerList,
    pub file_item_list_filter: FileItemListFilter,
    pub file_item_list_priority: FileItemListPriority,
    pub template_list: TemplateList,
    pub create_template: CreateTemplate,
    pub create_scheduler: CreateScheduler,
}

impl AppState {
    pub fn init() -> Result<Self, std::io::Error> {
        Ok(AppState {
            mode: AppMode::Tab,
            prev_mode: AppMode::Tab,
            flags: AppFlags::default(),
            components: AppComponents::init()?,
            exit: false,
        })
    }
}

impl AppComponents {
    pub fn init() -> Result<Self, std::io::Error> {
        Ok(AppComponents {
            tabs: TabComponent::init(),
            file_list: FileList::new()?,
            file_item_list_filter: FileItemListFilter::new(),
            file_item_list_priority: FileItemListPriority::new(),
            template_list: TemplateList::new(),
            scheduler_list: SchedulerList::new(),
            create_template: CreateTemplate::new(String::new()),
            create_scheduler: CreateScheduler::new(
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new()
            ),
        })
    }
}
