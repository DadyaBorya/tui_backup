use crate::models::dir_entry::DirEntry;

pub fn dir_entry_to_template(entry: &DirEntry) -> String {
    let mut content = "".to_string();

    dir_entry_to_template_recursive(entry, &mut content);

    content
}

fn dir_entry_to_template_recursive(entry: &DirEntry, content: &mut String) {
    let path: String = path(entry);

    if !path.is_empty() {
        let file_filter = file_filter(entry);
        let dir_filter = dir_filter(entry);
        let dir_file_priority = dir_file_priority(entry);
        let dir_priority = dir_priority(entry);
        let file_priority = file_priority(entry);

        let selected = match entry.selected {
            true => String::from("s"),
            false => String::from(""),
        };

        content.push_str(
            &format!(
                "{}>{}{}{}{}{}{}\n",
                path,
                file_filter,
                dir_filter,
                dir_file_priority,
                dir_priority,
                file_priority,
                selected
            )
        );
    }

    if let Some(children) = entry.children.as_ref() {
        for child in children {
            dir_entry_to_template_recursive(child, content);
        }
    }
}

fn path(entry: &DirEntry) -> String {
    let entry_path = entry.path();

    if entry.selected {
        return entry_path;
    }

    if let Some(filters) = entry.entry_file_filter.as_ref() {
        for filter in filters.iter() {
            if filter.root == entry_path {
                return entry_path;
            }
        }
    }

    if let Some(filters) = entry.entry_dir_filter.as_ref() {
        for filter in filters.iter() {
            if filter.root == entry_path {
                return entry_path;
            }
        }
    }

    if let Some(priorities) = entry.entry_dir_file_priority.as_ref() {
        for priority in priorities.iter() {
            if priority.root == entry_path {
                return entry_path;
            }
        }
    }

    if let Some(priorities) = entry.entry_dir_priority.as_ref() {
        for priority in priorities.iter() {
            if priority.root == entry_path {
                return entry_path;
            }
        }
    }

    if let Some(priorities) = entry.entry_file_priority.as_ref() {
        for priority in priorities.iter() {
            if priority.root == entry_path {
                return entry_path;
            }
        }
    }

    String::new()
}

fn file_filter(entry: &DirEntry) -> String {
    let mut strings = vec![];
    let entry_path = entry.path();
    if let Some(filters) = entry.entry_file_filter.as_ref() {
        for filter in filters.iter() {
            if filter.root == entry_path {
                strings.push(format!("{{{}, {}, {}}}", filter.regex, filter.deep, filter.content));
            }
        }
        return format!("ff[{}]", strings.join(","));
    }
    String::new()
}

fn dir_filter(entry: &DirEntry) -> String {
    let mut strings = vec![];
    let entry_path = entry.path();
    if let Some(filters) = entry.entry_dir_filter.as_ref() {
        for filter in filters.iter() {
            if filter.root == entry_path {
                strings.push(format!("{{{}, {}}}", filter.regex, filter.deep));
            }
        }
        return format!("df[{}]", strings.join(","));
    }
    String::new()
}

fn dir_file_priority(entry: &DirEntry) -> String {
    let mut strings = vec![];
    let entry_path = entry.path();
    if let Some(priorities) = entry.entry_dir_file_priority.as_ref() {
        for priority in priorities.iter() {
            if priority.root == entry_path {
                strings.push(
                    format!(
                        "{{{}, {}, {}, {}}}",
                        priority.regex,
                        priority.deep,
                        priority.priority,
                        priority.content
                    )
                );
            }
        }
        return format!("dfp[{}]", strings.join(","));
    }
    String::new()
}

fn dir_priority(entry: &DirEntry) -> String {
    let mut strings = vec![];
    let entry_path = entry.path();
    if let Some(priorities) = entry.entry_dir_priority.as_ref() {
        for priority in priorities.iter() {
            if priority.root == entry_path {
                strings.push(
                    format!("{{{}, {}, {}}}", priority.regex, priority.deep, priority.priority)
                );
            }
        }
        return format!("dp[{}]", strings.join(","));
    }
    String::new()
}

fn file_priority(entry: &DirEntry) -> String {
    let mut strings = vec![];
    let entry_path = entry.path();
    if let Some(priorities) = entry.entry_file_priority.as_ref() {
        for priority in priorities.iter() {
            if priority.root == entry_path {
                strings.push(format!("{{{}, {}}}", priority.priority, priority.content));
            }
        }
        return format!("fp[{}]", strings.join(","));
    }
    String::new()
}
