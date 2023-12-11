#[derive(PartialEq, Clone)]
pub enum AppMode {
    Tab,
    FileList,
    MessagePopup,
    FileFilter,
    DirFilter,
    DirPriority,
    DirFilePriority,
    FilePriority,
    FileFilterForm(FileFilterForm),
}

#[derive(PartialEq, Clone)]
pub enum FileFilterForm {
    Regex,
    Content,
    Deep,
    Submit,
}
