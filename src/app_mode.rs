#[derive(PartialEq)]
pub enum AppMode {
    Tab,
    FileList,
    ErrorPopup,
    FolderListFilter(FolderListFilter),
    FileFolderListFilter(FileFolderListFilter),
    FolderListPriority(FolderListPriority),
    FileFolderListPriority(FileFolderListPriority),
    FileListPriority(FileListPriority),
}

#[derive(PartialEq, Clone)]
pub enum FolderListFilter {
    List,
    Form,
    Regex,
    Deep,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum FileFolderListFilter {
    List,
    Form,
    Regex,
    Deep,
    Content,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum FolderListPriority {
    List,
    Form,
    Regex,
    Deep,
    Priority,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum FileFolderListPriority {
    List,
    Form,
    Regex,
    Deep,
    Content,
    Priority,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum FileListPriority {
    List,
    Form,
    Content,
    Priority,
    Submit,
}
