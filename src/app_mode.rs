#[derive(PartialEq)]
pub enum AppMode {
    Tab,
    FileList,
    ErrorPopup,
    FolderListFilter(FolderListFilter),
    FileFolderListFilter(FileFolderListFilter)
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