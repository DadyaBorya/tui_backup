use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::components::{
    dir::priority::{
            dir::form::component::DirPriorityFormComponent,
            file::form::component::DirFilePriorityFormComponent,
        },
    file::{
        filter::form::component::FileFilterFormComponent, list::{component::FileListComponent, settings::component::FileListSettingComponent},
        priority::form::component::FilePriorityFormComponent,
    },
    helper::helper_ui,
    popup::message::component::MessagePopupComponent,
    scheduler::{form::component::SchedulerFormComponent, list::component::SchedulerListComponent},
    tab::component::TabComponent,
    template::{form::component::TemplateFormComponent, list::component::TemplateListComponent},
};

use super::{app::App, mode::AppMode};

impl App {
    pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
        let chunks = App::get_main_chunks(f.size());

        TabComponent::ui(app, f, &chunks);

        App::preview(app, f, &chunks);

        App::handle_current_app_mode(app, f, &chunks);

        helper_ui::ui(app, f, &chunks);
    }

    fn get_main_chunks(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area)
    }

    fn preview<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        let index: usize = app.components.tabs.state.index;

        match index {
            0 => FileListComponent::ui(app, f, &chunks),
            1 => TemplateListComponent::ui(app, f, &chunks),
            2 => SchedulerListComponent::ui(app, f, &chunks),
            _ => {}
        }
    }

    fn handle_current_app_mode<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
        match app.state.mode {
            AppMode::Tab => TabComponent::ui(app, f, &chunks),
            AppMode::MessagePopup => MessagePopupComponent::ui(f, app),
            AppMode::FileListSettings => FileListSettingComponent::ui(app, f),
            AppMode::FileFilterForm(_) => FileFilterFormComponent::ui(app, f),
            AppMode::DirFilePriorityForm(_) => DirFilePriorityFormComponent::ui(app, f),
            AppMode::DirPriorityForm(_) => DirPriorityFormComponent::ui(app, f),
            AppMode::FilePriorityForm(_) => FilePriorityFormComponent::ui(app, f),
            AppMode::TemplateForm(_) => TemplateFormComponent::ui(app, f),
            AppMode::SchedulerForm(_) => SchedulerFormComponent::ui(app, f),
            _ => {}
        }
    }
}
