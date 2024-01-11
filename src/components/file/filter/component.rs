use crate::{
    application::{
        app::App,
        mode::{AppMode, FileFilterForm},
    },
    components::popup::message::component::MessagePopupComponent,
    utils::list_utils, models::entry_file_filter::EntryFileFilter,
};

use super::state::FileFilterState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | ENTER~Select | n~New | d~Delete | e~Edit |";

pub struct FileFilterComponent {
    pub state: FileFilterState,
}

impl FileFilterComponent {
    pub fn init() -> Self {
        FileFilterComponent {
            state: FileFilterState::init(),
        }
    }

    pub fn exit(app: &mut App) {
        let file_filter = &mut app.components.file_filter;

        if let Some(_) = file_filter.state.list_state.selected() {
            file_filter.state.list_state.select(None);
            return;
        }

        
        app.change_mode(AppMode::FileList, AppMode::FileFilter);
    }

    pub fn select_list(app: &mut App) {
        let file_filter = &mut app.components.file_filter;

        match file_filter.state.list_state.selected() {
            None => file_filter.state.init_index_table(),
            _ => {}
        };
    }

    pub fn next_component(app: &mut App) {
        let file_filter = &mut app.components.file_filter;
        let mut settings = app
            .components
            .file_list_settings
            .state
            .seleted_items
            .clone();

        let is_selected = match file_filter.state.list_state.selected() {
            Some(_) => true,
            None => false,
        };

        if !is_selected {
            settings.retain(|i| i != &3);
            settings.sort();

            if settings.len() > 1 {
                match settings[1] {
                    1 => app.change_mode(AppMode::DirFilePriority, app.state.prev_mode.clone()),
                    2 => app.change_mode(AppMode::DirPriority, app.state.prev_mode.clone()),
                    _ => {}
                }
            }
        }
    }

    pub fn move_up(app: &mut App) {
        let file_filter = &mut app.components.file_filter;

        match file_filter.state.list_state.selected() {
            Some(_) => list_utils::move_up(
                &mut file_filter.state.list_state,
                file_filter.state.rules.len(),
            ),
            None => {}
        };
    }

    pub fn move_down(app: &mut App) {
        let file_filter = &mut app.components.file_filter;

        match file_filter.state.list_state.selected() {
            Some(_) => list_utils::move_down(
                &mut file_filter.state.list_state,
                file_filter.state.rules.len(),
            ),
            None => FileFilterComponent::next_component(app),
        };
    }

    pub fn delete(app: &mut App) {
        if let Some(entry) = app.components.file_list.state.get_selected_entry() {
            if let Some(index) = app.components.file_filter.state.list_state.selected() {
                let filter_root = entry.entry_file_filter.as_ref().unwrap()[index]
                    .root
                    .clone();

                if filter_root != entry.path() {
                    MessagePopupComponent::show(
                        app,
                        "Can't delete root filter".to_string(),
                        format!("Root filter is {}", filter_root),
                    );
                    return;
                }

                entry.entry_file_filter.as_mut().unwrap().remove(index);

                if entry.entry_file_filter.as_ref().unwrap().len() == 0 {
                    entry.entry_file_filter = None;
                }

                app.components.file_filter.state.rules.remove(index);
                FileFilterComponent::move_up(app);
            }
        }
    }

    pub fn edit(app: &mut App) {
        if let Some(index) = app.components.file_filter.state.list_state.selected() {
            let filter = app.components.file_filter.state.rules[index].clone();

            let entry = app.components.file_list.state.get_selected_entry().unwrap();

            if filter.root != entry.path() {
                MessagePopupComponent::show(
                    app,
                    "Can't edit root filter".to_string(),
                    format!("Root filter is {}", filter.root),
                );
                return;
            }

            let state = &mut app.components.file_filter_form.state;
            state.regex = filter.regex;
            state.content = filter.content;
            state.deep = EntryFileFilter::get_deep(filter.deep);
            app.components.file_filter.state.is_edit = true;
            app.change_mode(
                AppMode::FileFilterForm(FileFilterForm::Regex),
                app.state.prev_mode.clone(),
            );
        }
    }

    pub fn new_rule(app: &mut App) {
        app.change_mode(
            AppMode::FileFilterForm(FileFilterForm::Regex),
            AppMode::FileFilter,
        );
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
