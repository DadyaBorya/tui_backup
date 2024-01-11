use crate::{
    application::{
        app::App,
        mode::{AppMode, DirPriorityForm},
    },
    components::popup::message::component::MessagePopupComponent,
    utils::list_utils,
};

use super::state::DirPriorityState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | ENTER~Select | n~New | d~Delete | e~Edit |";

pub struct DirPriorityComponent {
    pub state: DirPriorityState,
}

impl DirPriorityComponent {
    pub fn init() -> Self {
        DirPriorityComponent {
            state: DirPriorityState::init(),
        }
    }

    pub fn delete(app: &mut App) {
        if let Some(entry) = app.components.file_list.state.get_selected_entry() {
            if let Some(index) = app.components.dir_priority.state.list_state.selected() {
                let priority_root = entry.entry_dir_priority.as_ref().unwrap()[index]
                    .root
                    .clone();

                if priority_root != entry.path() {
                    MessagePopupComponent::show(
                        app,
                        "Can't delete root priority".to_string(),
                        format!("Root priority is {}", priority_root),
                    );
                    return;
                }
                entry.entry_dir_priority.as_mut().unwrap().remove(index);

                if entry.entry_dir_priority.as_ref().unwrap().is_empty() {
                    entry.entry_dir_priority = None;
                }

                app.components.dir_priority.state.rules.remove(index);
                DirPriorityComponent::move_up(app);
            }
        }
    }

    pub fn exit(app: &mut App) {
        let dir_priority = &mut app.components.dir_priority;

        if let Some(_) = dir_priority.state.list_state.selected() {
            dir_priority.state.list_state.select(None);
            return;
        }

        
        app.change_mode(AppMode::FileList, AppMode::DirPriority);
    }

    pub fn select_list(app: &mut App) {
        let dir_priority = &mut app.components.dir_priority;

        match dir_priority.state.list_state.selected() {
            None => dir_priority.state.init_index_table(),
            _ => {}
        };
    }

    pub fn edit(app: &mut App) {
        if let Some(index) = app.components.dir_priority.state.list_state.selected() {
            let priority = app.components.dir_priority.state.rules[index].clone();

            let entry = app.components.file_list.state.get_selected_entry().unwrap();

            if priority.root != entry.path() {
                MessagePopupComponent::show(
                    app,
                    "Can't delete root priority".to_string(),
                    format!("Root priority is {}", priority.root),
                );
                return;
            }
            let state = &mut app.components.dir_priority_form.state;
            state.regex = priority.regex;
            state.deep = priority.deep.to_string();
            state.priority = priority.priority.to_string();
            app.components.dir_priority.state.is_edit = true;
            app.change_mode(
                AppMode::DirPriorityForm(DirPriorityForm::Regex),
                app.state.prev_mode.clone(),
            );
        }
    }

    pub fn move_up(app: &mut App) {
        let dir_priority = &mut app.components.dir_file_priority;
        match dir_priority.state.list_state.selected() {
            Some(_) => list_utils::move_up(
                &mut dir_priority.state.list_state,
                dir_priority.state.rules.len(),
            ),
            None => DirPriorityComponent::prev_component(app),
        };
    }

    pub fn move_down(app: &mut App) {
        let dir_priority = &mut app.components.dir_file_priority;
        match dir_priority.state.list_state.selected() {
            Some(_) => list_utils::move_down(
                &mut dir_priority.state.list_state,
                dir_priority.state.rules.len(),
            ),
            None => {}
        };
    }

    pub fn prev_component(app: &mut App) {
        let mut settings = app
            .components
            .file_list_settings
            .state
            .seleted_items
            .clone();

        settings.retain(|i| i != &3);
        settings.sort();

        if settings.len() > 1 {
            let index = settings.iter().position(|i| i == &2).unwrap() - 1;
            match settings[index] {
                0 => app.change_mode(AppMode::FileFilter, app.state.prev_mode.clone()),
                1 => app.change_mode(AppMode::DirFilePriority, app.state.prev_mode.clone()),
                _ => {}
            }
        }
    }

    pub fn new_rule(app: &mut App) {
        app.change_mode(
            AppMode::DirPriorityForm(DirPriorityForm::Regex),
            AppMode::DirPriority,
        );
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
