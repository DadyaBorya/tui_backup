#[derive(PartialEq, Clone)]
pub enum AppMode {
    Tab,
    FileList,
    MessagePopup,
    FileFilter,
    DirFilter,
    FilePriority,
    DirPriority,
    DirFilePriority
    // ErrorPopup,
    // HelpPopup,
    // TemplateList,
    // SchedulerList,
    // FolderListFilter(FolderListFilter),
    // FileFolderListFilter(FileFolderListFilter),
    // FolderListPriority(FolderListPriority),
    // FileFolderListPriority(FileFolderListPriority),
    // FileListPriority(FileListPriority),
    // CreateTemplate(CreateTemplate),
    // CreateScheduler(CreateScheduler),
}

// #[derive(PartialEq, Clone)]
// pub enum FolderListFilter {
//     List,
//     Form,
//     Regex,
//     Deep,
//     Submit,
// }

// #[derive(PartialEq, Clone)]
// pub enum FileFolderListFilter {
//     List,
//     Form,
//     Regex,
//     Deep,
//     Content,
//     Submit,
// }

// #[derive(PartialEq, Clone)]
// pub enum FolderListPriority {
//     List,
//     Form,
//     Regex,
//     Deep,
//     Priority,
//     Submit,
// }

// #[derive(PartialEq, Clone)]
// pub enum FileFolderListPriority {
//     List,
//     Form,
//     Regex,
//     Deep,
//     Content,
//     Priority,
//     Submit,
// }

// #[derive(PartialEq, Clone)]
// pub enum FileListPriority {
//     List,
//     Form,
//     Content,
//     Priority,
//     Submit,
// }

// #[derive(PartialEq, Clone)]
// pub enum CreateTemplate {
//     Form,
//     Name,
//     Submit,
// }

// #[derive(PartialEq, Clone)]
// pub enum CreateScheduler {
//     Form,
//     Name,
//     Clouds,
//     Protocols,
//     Cron,
//     SpeedLimit,
//     Submit,
// }
