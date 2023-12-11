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
    DirFilterForm(DirFilterForm),
}

#[derive(PartialEq, Clone)]
pub enum FileFilterForm {
    Regex,
    Content,
    Deep,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum DirFilterForm {
    Regex,
    Deep,
    Submit,
}
