use std::path::{ Path, PathBuf };

use crate::models::dir_entry::DirEntry;

pub fn find_in_dir<'a>(dir: &'a mut DirEntry, path: &Path) -> Option<&'a mut DirEntry> {
    if dir.path.as_path() == path {
        return Some(dir);
    }

    if let Some(children) = dir.children.as_mut() {
        for child in children {
            if child.path.as_path() == path {
                return Some(child);
            } else if child.children.is_none() && child.is_dir() {
                continue;
            } else if path.starts_with(child.path.as_path()) && child.is_dir() {
                if let Some(entry) = find_in_dir(child, path) {
                    return Some(entry);
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

pub fn add_vec_items(root: &mut DirEntry, entries: Vec<DirEntry>) {
    for entry in entries {
        let mut components: Vec<PathBuf> = entry.path
            .ancestors()
            .map(|ancestor| ancestor.to_path_buf())
            .collect();
        components.reverse();

        for (index, component) in components.iter().enumerate() {
            if let Some(find_entry) = find_in_dir(root, &component) {
                if find_entry.children.is_none() && index != components.len() - 1 {
                    let _ = find_entry.renew_children();
                }

                if index == components.len() - 1 {
                    *find_entry = entry.clone();
                }
            }
        }
    }
}
