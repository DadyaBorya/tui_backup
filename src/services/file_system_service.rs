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

pub fn add_existing_items(entry: &mut DirEntry, entries: Vec<DirEntry>) {
    entries.iter().for_each(|e| add_existing_item(entry, e.clone()));
}

pub fn add_existing_item(entry: &mut DirEntry, new_entry: DirEntry) {
    if entry.children.is_none() {
        entry.children = Some(Vec::new());
    }

    let is_exist = entry.children
        .as_ref()
        .unwrap()
        .iter()
        .any(|e| e.path.as_path() == new_entry.path.as_path());

    if !is_exist {
        entry.children.as_mut().unwrap().push(new_entry);
    }
}


