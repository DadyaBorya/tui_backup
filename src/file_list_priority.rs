#[derive(Debug, Clone)]
pub struct FolderPriority {
    pub regex: String,
    pub deep: String,
    pub priority: String
}

#[derive(Debug, Clone)]
pub struct FileFolderPriority {
    pub regex: String,
    pub deep: String,
    pub content: String,
    pub priority: String
}

#[derive(Debug, Clone)]
pub struct FilePriority {
    pub regex: String,
    pub priority: String
} 