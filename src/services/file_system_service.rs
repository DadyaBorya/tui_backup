use std::path::Path;

use crate::models::dir_entry::DirEntry;
pub fn find_in_dir<'a>(dir: &'a mut DirEntry, path: &Path) -> Option<&'a mut DirEntry> {
    if dir.path.as_path() == path {
        return Some(dir);
    }

    if let Some(children) = dir.children.as_mut() {
        for child in children {
            if child.is_dir() {
                if child.path.as_path() == path {
                    return Some(child);
                } else if child.children.is_none() {
                    continue;
                } else if path.starts_with(child.path.as_path()) {
                    if let Some(entry) = find_in_dir(child, path) {
                        return Some(entry);
                    }
                }
            }
        }
    }

    None
}
