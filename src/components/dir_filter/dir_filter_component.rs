use crate::{ application::{ app::App, app_mode::{ AppMode, DirFilterForm } }, utils::list_utils, components::message_popup::message_popup_components::MessagePopupComponent };

use super::dir_filter_state::DirFilterState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | [~Prev |";

pub struct DirFilterComponent {
    pub state: DirFilterState,
}

impl DirFilterComponent {
    pub fn init() -> Self {
        DirFilterComponent {
            state: DirFilterState::init(),
        }
    }

    pub fn move_up(&mut self) {
        list_utils::move_up(&mut self.state.list_state, self.state.rules.len());
    }

    pub fn move_down(&mut self) {
        list_utils::move_down(&mut self.state.list_state, self.state.rules.len());
    }

    pub fn exit(app: &mut App) {
        let dir_filter = &mut app.components.dir_filter;
        dir_filter.state.list_state.select(None);
        app.change_mode(AppMode::FileList, AppMode::DirFilter);
    }

    pub fn delete(app: &mut App) {
        if let Some(entry) = app.components.file_list.state.get_selected_entry() {
            if let Some(index) = app.components.dir_filter.state.list_state.selected() {
                let filter_root = entry.entry_dir_filter.as_ref().unwrap()[index].root.clone();

                if filter_root != entry.path() {
                    MessagePopupComponent::show(
                        app,
                        "Can't delete root filter".to_string(),
                        format!("Root filter is {}", filter_root)
                    );
                    return;
                }

                entry.entry_dir_filter.as_mut().unwrap().remove(index);

                if entry.entry_dir_filter.as_ref().unwrap().len() == 0 {
                    entry.entry_dir_filter = None;
                }

                app.components.dir_filter.state.rules.remove(index);
                app.components.dir_filter.move_up();
            }
        }
    }

    pub fn edit(app: &mut App) {
        if let Some(index) = app.components.dir_filter.state.list_state.selected() {
            let filter = app.components.dir_filter.state.rules[index].clone();

            let entry = app.components.file_list.state.get_selected_entry().unwrap();

            if filter.root != entry.path() {
                MessagePopupComponent::show(
                        app,
                        "Can't edit root filter".to_string(),
                        format!("Root filter is {}", filter.root)
                    );
                return;
            }

            let state = &mut app.components.dir_filter_form.state;
            state.regex = filter.regex;
            state.deep = filter.deep.to_string();
            app.components.dir_filter.state.is_edit = true;
            app.change_mode(
                AppMode::DirFilterForm(DirFilterForm::Regex),
                app.state.prev_mode.clone()
            );
        }
    }

    pub fn prev_component(app: &mut App) {
        let file_filter = &mut app.components.file_filter;
        file_filter.state.init_index_table();
        let dir_filter = &mut app.components.dir_filter;
        dir_filter.state.list_state.select(None);
        app.change_mode(AppMode::FileFilter, AppMode::DirFilter);
    }

    pub fn new_rule(app: &mut App) {
        app.change_mode(AppMode::DirFilterForm(DirFilterForm::Regex), AppMode::DirFilter);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
