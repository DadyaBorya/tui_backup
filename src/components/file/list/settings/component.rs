use crate::{application::{app::App, mode::AppMode}, utils::list_utils};

use super::state::FileListSettingState;

#[derive(Default)]
pub struct FileListSettingComponent {
    pub state: FileListSettingState,
}

impl FileListSettingComponent {
    pub fn init() -> Self {
        FileListSettingComponent {
            state: FileListSettingState::init(),
        }
    }

    pub fn move_up(&mut self) {
        let len = self.state.rows().len();
        list_utils::move_up(&mut self.state.list_state, len);
    }

    pub fn select(&mut self) {
        let state = &mut self.state;

        if let Some(index) = state.list_state.selected() {
            match state.seleted_items.contains(&index) {
                true => {
                    state.seleted_items.retain(|i| i != &index)
                },
                false => {
                    state.seleted_items.push(index)
                },
            }
        }
    }

    pub fn move_down(&mut self) {
        let len = self.state.rows().len();
        list_utils::move_down(&mut self.state.list_state, len);
    }

    pub fn exit(app: &mut App) {
        app.change_mode(AppMode::FileList, app.state.prev_mode.clone())
    }
}
