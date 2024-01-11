use regex::Regex;

use crate::models::{
    dir_entry::DirEntry,
    entry_dir_priority::EntryDirPriority,
    entry_dir_file_priority::EntryDirFilePriority,
    entry_file_priority::EntryFilePriority,
};

pub fn priority(entry: &mut DirEntry) {
    set_up_dir_priority(entry);

    set_up_dir_file_priority(entry);
}

pub fn delete_not_root(entry: &mut DirEntry) {
    delete_not_root_dir_priority(entry);
    delete_not_root_dir_file_priority(entry);
}

pub fn set_up_dir_priority(entry: &mut DirEntry) {
    if
        let (Some(children), Some(priorities)) = (
            entry.children.as_mut(),
            entry.entry_dir_priority.as_ref(),
        )
    {
        for child in children.iter_mut().filter(|child| child.is_dir()) {
            for priority in priorities {
                let priority_deep = match priority.deep {
                    Some(value) => {
                        if value == 0 {
                            continue;
                        }

                        Some(value - 1)
                    },
                    None => None,
                };

                let new_priority = EntryDirPriority {
                    regex: priority.regex.clone(),
                    deep: priority_deep,
                    priority: priority.priority,
                    root: priority.root.clone(),
                };

                let entry_priorities = child.entry_dir_priority.get_or_insert_with(Vec::new);

                if !entry_priorities.contains(&new_priority) {
                    entry_priorities.push(new_priority);
                }
            }
        }
    }
}

pub fn delete_not_root_dir_priority(entry: &mut DirEntry) {
    if let Some(children) = &mut entry.children {
        for child in children {
            let child_path = child.path();

            if let Some(priorities) = &mut child.entry_dir_priority {
                priorities.retain(|priority| priority.root == child_path);

                if priorities.is_empty() {
                    child.entry_dir_priority = None;
                }
            }
        }
    }
}

pub fn set_up_dir_file_priority(entry: &mut DirEntry) {
    if
        let (Some(children), Some(priorities)) = (
            entry.children.as_mut(),
            entry.entry_dir_file_priority.as_ref(),
        )
    {
        for child in children.iter_mut().filter(|child| child.is_dir()) {
            for priority in priorities {
                let priority_deep = match priority.deep {
                    Some(value) => {
                        if value == 0 {
                            continue;
                        }

                        Some(value - 1)
                    },
                    None => None,
                };

                let new_priority = EntryDirFilePriority {
                    regex: priority.regex.clone(),
                    deep: priority_deep,
                    priority: priority.priority,
                    content: priority.content.clone(),
                    root: priority.root.clone(),
                };

                let entry_priorities = child.entry_dir_file_priority.get_or_insert_with(Vec::new);

                if !entry_priorities.contains(&new_priority) {
                    entry_priorities.push(new_priority);
                }
            }
        }

        for child in children.iter_mut().filter(|child| !child.is_dir()) {
            for priority in priorities {
                let regex = Regex::new(&priority.regex).unwrap();

                if !regex.is_match(&child.file_name()) {
                    continue;
                }

                let new_priority = EntryFilePriority {
                    priority: priority.priority,
                    content: priority.content.clone(),
                    root: priority.root.clone(),
                };

                let entry_priorities = child.entry_file_priority.get_or_insert_with(Vec::new);

                if !entry_priorities.contains(&new_priority) {
                    entry_priorities.push(new_priority);
                }
            }
        }
    }
}

pub fn delete_not_root_dir_file_priority(entry: &mut DirEntry) {
    if let Some(children) = &mut entry.children {
        for child in children {
            let child_path = child.path();

            if let Some(priorities) = &mut child.entry_dir_file_priority {
                priorities.retain(|priority| priority.root == child_path);

                if priorities.is_empty() {
                    child.entry_dir_file_priority = None;
                }
            }
        }
    }
}
