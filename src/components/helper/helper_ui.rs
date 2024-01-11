use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::application::{app::App, mode::AppMode};

pub fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>, chunks: &Vec<Rect>) {
    let helper_text = get_helper_text(&app);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let paragraph = Paragraph::new(helper_text)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, chunks[2]);
}

fn get_helper_text(app: &App) -> &'static str {
    let mode = &app.state.mode;

    match mode {
        AppMode::Tab => app.components.tabs.get_helper_text(),
        AppMode::FileList => app.components.file_list.get_helper_text(),
        AppMode::MessagePopup => app.components.message_popup.get_helper_text(),
        AppMode::FileFilter => app.components.file_filter.get_helper_text(),
        AppMode::FilePriority => app.components.file_priority.get_helper_text(),
        AppMode::DirPriority => app.components.dir_priority.get_helper_text(),
        AppMode::DirFilePriority => app.components.dir_file_priority.get_helper_text(),
        AppMode::FileFilterForm(mode) => app.components.file_filter_form.get_helper_text(&mode),
        AppMode::DirFilePriorityForm(mode) => {
            app.components.dir_file_priority_form.get_helper_text(&mode)
        }
        AppMode::DirPriorityForm(mode) => app.components.dir_priority_form.get_help_text(&mode),
        AppMode::FilePriorityForm(mode) => app.components.file_priority_form.get_help_text(&mode),
        AppMode::TemplateForm(mode) => app.components.template_form.get_helper_text(&mode),
        AppMode::TemplateList => app.components.template_list.get_helper_text(),
        AppMode::SchedulerForm(mode) => app.components.scheduler_form.get_helper_text(&mode),
        AppMode::SchedulerList => app.components.scheduler_list.get_helper_text(),
        AppMode::FileListSettings => app.components.file_list_settings.get_helper_text(),
        AppMode::Confirm(mode) => app.components.confirm.get_helper_text(&mode),
    }
}
