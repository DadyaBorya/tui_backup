use regex::Regex;

use crate::models::{
    dir_entry::DirEntry,
    entry_file_filter::EntryFileFilter,
};

use super::file_service;

pub fn filter(entry: &mut DirEntry) {
    set_up_dir_file_filter(entry);
    apply_dir_file_filter(entry);
}

pub fn delete_not_root(entry: &mut DirEntry) {
    delete_not_root_dir_file_filter(entry);
}

pub fn set_up_dir_file_filter(entry: &mut DirEntry) {
    if
        let (Some(children), Some(filters)) = (
            entry.children.as_mut(),
            entry.entry_file_filter.as_ref(),
        )
    {
        for child in children.iter_mut().filter(|child| child.is_dir()) {
            for filter in filters {
                if filter.deep == 0 {
                    continue;
                }

                let new_filter = EntryFileFilter {
                    regex: filter.regex.clone(),
                    content: filter.content.clone(),
                    deep: filter.deep - 1,
                    root: filter.root.clone(),
                };

                let entry_filters = child.entry_file_filter.get_or_insert_with(Vec::new);

                if !entry_filters.contains(&new_filter) {
                    entry_filters.push(new_filter);
                }
            }
        }
    }
}

pub fn delete_not_root_dir_file_filter(entry: &mut DirEntry) {
    if let Some(children) = &mut entry.children {
        for child in children {
            let child_path = child.path();

            if let Some(filters) = &mut child.entry_file_filter {
                filters.retain(|filter| filter.root == child_path);
                if filters.is_empty() {
                    child.entry_file_filter = None;
                }
            }
        }
    }
}

pub fn apply_dir_file_filter(entry: &mut DirEntry) {
    if
        let (Some(children), Some(filters)) = (
            entry.children.as_mut(),
            entry.entry_file_filter.as_ref(),
        )
    {
        children.retain(|child| {
            if !child.is_dir() {
                filters.iter().any(|filter| {
                    let regex = Regex::new(&filter.regex).unwrap();
                    {
                        match regex.is_match(&child.file_name()) {
                            true => {
                                if filter.content.is_empty() {
                                    return true;
                                }

                                if let Ok(content) = file_service::read_file(&child.path) {
                                    let regex = Regex::new(&filter.content).unwrap();

                                    return regex.is_match(&content);
                                }
                                false
                            }
                            false => false,
                        }
                    }
                })
            } else {
                true
            }
        });
    }
}
