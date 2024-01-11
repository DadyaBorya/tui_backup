use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    application::{app::App, mode::AppMode},
    generator::{list_generator, popup},
};

use super::component::FileListSettingComponent;

impl FileListSettingComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let state = &mut app.components.file_list_settings.state;

        let area = popup::popup(50, 50, "Settigs".to_string(), f);

        let chunks = Layout::default()
            .margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let settings_list = list_generator::list(
            "".to_string(),
            app.state.mode == AppMode::FileListSettings,
            state.rows(),
        );

        f.render_stateful_widget(settings_list, chunks[0], &mut state.list_state);
    }
}
