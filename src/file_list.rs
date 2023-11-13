use crate::file_system::FileSystem;

#[derive(Debug)]
pub struct FileList {
    pub root: FileSystem
}

impl FileList {
    pub fn new() -> Self {
        FileList {
            root: FileSystem::new()
        }
    }
}