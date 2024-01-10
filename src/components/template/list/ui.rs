use tui::{ backend::Backend, Frame, layout::Rect };

use crate::{ application::{ app::App, mode::AppMode }, generator::list_generator };

use super::component::TemplateListComponent;

impl TemplateListComponent {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let template_list = &mut app.components.template_list;
    
        let list = list_generator::list(
            "Template List".to_string(),
            AppMode::TemplateList == app.state.mode,
            template_list.state.rows()
        );
    
        f.render_stateful_widget(list, chunks[1], &mut template_list.state.list_state);
    }
    
}