#[derive(PartialEq, Clone)]
pub enum AppMode {
    Tab,
    FileList,
    MessagePopup,
    FileFilter,
    DirPriority,
    DirFilePriority,
    FilePriority,
    FileFilterForm(FileFilterForm),
    DirFilePriorityForm(DirFilePriorityForm),
    DirPriorityForm(DirPriorityForm),
    FilePriorityForm(FilePriorityForm),
    TemplateForm(TemplateForm),
    TemplateList,
    SchedulerForm(SchedulerForm),
    SchedulerList,
    FileListSettings
}

#[derive(PartialEq, Clone)]
pub enum FileFilterForm {
    Regex,
    Content,
    Deep,
    Submit,
}


#[derive(PartialEq, Clone)]
pub enum DirFilePriorityForm {
    Regex,
    Deep,
    Priority,
    Content,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum DirPriorityForm {
    Regex,
    Deep,
    Priority,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum FilePriorityForm {
    Priority,
    Content,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum TemplateForm {
    Name,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum SchedulerForm {
    Name,
    Speed,
    NextCron,
    Cron,
    NextCloud,
    PrevName,
    Cloud,
    Protocol,
    PrevCron,
    Submit,
}
