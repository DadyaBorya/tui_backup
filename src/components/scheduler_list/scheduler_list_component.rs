use std::path::PathBuf;

use crate::{
    models::{ config::Config, scheduler::Scheduler },
    application::{ app::App, app_mode::{ AppMode, SchedulerForm } },
    utils::table_util,
    services::file_service,
};

use super::scheduler_list_state::SchedulerListState;

const HELP: &'static str = "| ESC~Back | ↑ ↓ Move | d~Delete | | e~Edit |";

pub struct SchedulerListComponent {
    pub state: SchedulerListState,
}

impl SchedulerListComponent {
    pub fn init(config: &Config) -> Result<Self, std::io::Error> {
        Ok(SchedulerListComponent {
            state: SchedulerListState::init(config)?,
        })
    }

    pub fn delete(&mut self) {
        if let Some(scheduler) = self.state.selected() {
            if let Ok(_) = file_service::delete_file(&PathBuf::from(scheduler)) {
                self.state.renew();
                self.move_up();
            }
        }
    }

    // SHIT CODE HERE
    pub fn edit(app: &mut App) {
        let scheduler_list_state: &mut SchedulerListState =
            &mut app.components.scheduler_list.state;

        if let Some(scheduler) = scheduler_list_state.selected() {
            if let Ok(json) = file_service::read_file(&PathBuf::from(scheduler)) {
                if let Ok(scheduler) = serde_json::from_str::<Scheduler>(&json) {
                    let scheduler_form_state = &mut app.components.scheduler_form.state;

                    scheduler_form_state.name = scheduler.name;
                    scheduler_form_state.cron = scheduler.cron;
                    scheduler_form_state.speed = scheduler.speed.to_string();
                    scheduler_form_state.root = scheduler.root;

                    scheduler_form_state.fill_cloud_protocols();

                    for (k, v) in scheduler.clouds.iter() {
                        for (c, p) in scheduler_form_state.clouds_protocols.iter_mut() {
                            if k == &c.0 {
                                c.1 = true;

                                for sch_p in v.iter() {
                                    for s_p in p.iter_mut() {
                                        if sch_p == &s_p.0 {
                                            s_p.1 = true;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    app.change_mode(
                        AppMode::SchedulerForm(SchedulerForm::Name),
                        app.state.prev_mode.clone()
                    )
                }
            }
        }
    }

    pub fn move_up(&mut self) {
        table_util::move_up(&mut self.state.table_state, self.state.schedulers.len());
    }

    pub fn move_down(&mut self) {
        table_util::move_down(&mut self.state.table_state, self.state.schedulers.len());
    }

    pub fn exit(app: &mut App) {
        let state = &mut app.components.scheduler_list.state;
        state.table_state.select(None);
        app.change_mode(AppMode::Tab, AppMode::SchedulerList);
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}
