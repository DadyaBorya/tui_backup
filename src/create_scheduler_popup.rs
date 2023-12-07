use crossterm::event::KeyCode;
use regex::Regex;
use tui::{
    backend::Backend,
    Frame,
    widgets::{ Block, Borders, BorderType, Clear },
    layout::{ Alignment, Layout, Direction, Constraint },
    style::{ Style, Color },
};

use crate::{
    app::App,
    popup::Popup,
    widget_gen::WidgetGen,
    app_mode::{ AppMode, CreateScheduler },
    scheduler::{ Scheduler, Cloud, Protocol },
    file_service,
};

pub struct CreateSchedulerPopup {}

impl CreateSchedulerPopup {
    pub fn map_to_scheduler(
        name: String,
        clouds: String,
        protocols: String,
        cron: String,
        speed_limit: String
    ) -> Result<Scheduler, ()> {
        if name.is_empty() {
            return Err(());
        }

        let name = name.trim().to_string();

        let speed_limit = match speed_limit.parse::<usize>() {
            Ok(limit) => limit,
            Err(_) => {
                return Err(());
            }
        };

        let regex_str =
            r"(@(annually|yearly|monthly|weekly|daily|hourly|reboot))|(@every (\d+(ns|us|µs|ms|s|m|h))+)|((((\d+,)+\d+|(\d+(\/|-)\d+)|\d+|\*) ?){5,7})";
        let regex = Regex::new(regex_str).unwrap();

        let cron = match regex.is_match(&cron) {
            true => cron,
            false => {
                return Err(());
            }
        };

        if clouds.is_empty() {
            return Err(());
        }

        let clouds = clouds
            .split(',')
            .flat_map(|c| {
                match Cloud::from_str(c.trim()) {
                    Some(cl) => Ok(cl),
                    None => Err(()),
                }
            })
            .collect::<Vec<Cloud>>();

        let protocols = protocols
            .split(',')
            .flat_map(|p| {
                match Protocol::from_str(p.trim()) {
                    Some(pt) => Ok(pt),
                    None => Err(()),
                }
            })
            .collect::<Vec<Protocol>>();

        Ok(
            Scheduler::new(
                name,
                String::new(),
                clouds,
                protocols,
                cron,
                speed_limit,
                String::new(),
                String::new()
            )
        )
    }

    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let block = Block::default()
            .title("Create Scheduler")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let area = Popup::centered_rect(60, 67, f.size());
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let chunks = Layout::default()
            .margin(2)
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Max(3),
                ].as_ref()
            )
            .split(area);

        let name_input = WidgetGen::form_input("Name", app.create_scheduler.name.as_str(), match
            app.mode
        {
            AppMode::CreateScheduler(CreateScheduler::Name) => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

        f.render_widget(name_input, chunks[0]);

        let cloud_input = WidgetGen::form_input(
            "Clouds",
            app.create_scheduler.clouds.as_str(),
            match app.mode {
                AppMode::CreateScheduler(CreateScheduler::Clouds) =>
                    Style::default().fg(Color::Yellow),
                _ => Style::default(),
            }
        );

        f.render_widget(cloud_input, chunks[1]);

        let protocol_input = WidgetGen::form_input(
            "Protocols",
            app.create_scheduler.protocols.as_str(),
            match app.mode {
                AppMode::CreateScheduler(CreateScheduler::Protocols) =>
                    Style::default().fg(Color::Yellow),
                _ => Style::default(),
            }
        );

        f.render_widget(protocol_input, chunks[2]);

        let cron_input = WidgetGen::form_input("Cron", app.create_scheduler.cron.as_str(), match
            app.mode
        {
            AppMode::CreateScheduler(CreateScheduler::Cron) => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

        f.render_widget(cron_input, chunks[3]);

        let speed_limit_input = WidgetGen::form_input(
            "Speed limit",
            app.create_scheduler.speed_limit.as_str(),
            match app.mode {
                AppMode::CreateScheduler(CreateScheduler::SpeedLimit) =>
                    Style::default().fg(Color::Yellow),
                _ => Style::default(),
            }
        );

        f.render_widget(speed_limit_input, chunks[4]);

        let submit_btn = WidgetGen::form_button("Submit", match app.mode {
            AppMode::CreateScheduler(CreateScheduler::Submit) => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

        f.render_widget(submit_btn, chunks[5])
    }

    pub fn event(app: &mut App, key_code: KeyCode) -> Result<(), std::io::Error> {
        match app.mode {
            AppMode::CreateScheduler(CreateScheduler::Form) => {
                match key_code {
                    KeyCode::Esc => {
                        app.create_scheduler.clear_inputs();
                        app.change_mode(AppMode::TemplateList);
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Name));
                    }
                    _ => {}
                }
            }
            AppMode::CreateScheduler(CreateScheduler::Name) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Form));
                    }
                    KeyCode::Char(c) => {
                        app.create_scheduler.name.push(c);
                    }
                    KeyCode::Backspace => {
                        app.create_scheduler.name.pop();
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Clouds));
                    }
                    _ => {}
                }
            }
            AppMode::CreateScheduler(CreateScheduler::Clouds) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Form));
                    }
                    KeyCode::Char(c) => {
                        app.create_scheduler.clouds.push(c);
                    }
                    KeyCode::Backspace => {
                        app.create_scheduler.clouds.pop();
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Protocols));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Name));
                    }
                    _ => {}
                }
            }
            AppMode::CreateScheduler(CreateScheduler::Protocols) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Form));
                    }
                    KeyCode::Char(c) => {
                        app.create_scheduler.protocols.push(c);
                    }
                    KeyCode::Backspace => {
                        app.create_scheduler.protocols.pop();
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Cron));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Clouds));
                    }
                    _ => {}
                }
            }
            AppMode::CreateScheduler(CreateScheduler::Cron) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Form));
                    }
                    KeyCode::Char(c) => {
                        app.create_scheduler.cron.push(c);
                    }
                    KeyCode::Backspace => {
                        app.create_scheduler.cron.pop();
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::SpeedLimit));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Protocols));
                    }
                    _ => {}
                }
            }
            AppMode::CreateScheduler(CreateScheduler::SpeedLimit) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Form));
                    }
                    KeyCode::Char(c) => {
                        app.create_scheduler.speed_limit.push(c);
                    }
                    KeyCode::Backspace => {
                        app.create_scheduler.speed_limit.pop();
                    }
                    KeyCode::Tab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Submit));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Cron));
                    }
                    _ => {}
                }
            }
            AppMode::CreateScheduler(CreateScheduler::Submit) => {
                match key_code {
                    KeyCode::Esc => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::Form));
                    }
                    KeyCode::BackTab => {
                        app.change_mode(AppMode::CreateScheduler(CreateScheduler::SpeedLimit));
                    }
                    KeyCode::Enter => {
                        if
                            let Ok(mut scheduler) = CreateSchedulerPopup::map_to_scheduler(
                                app.create_scheduler.name.to_owned(),
                                app.create_scheduler.clouds.to_owned(),
                                app.create_scheduler.protocols.to_owned(),
                                app.create_scheduler.cron.to_owned(),
                                app.create_scheduler.speed_limit.to_owned()
                            )
                        {
                            let index = app.template_list.list_state.selected().unwrap();
                            let file_name = &app.template_list.templates[index];
                            let path = format!("templates/{}.json", file_name);
                            scheduler.template_path = path.clone();
                            scheduler.out_path = format!("{}.json", file_name);

                            file_service::save_scheduler(
                                &scheduler,
                                &format!("schedulers/{}.json", &scheduler.name)
                            )?;

                            app.create_scheduler.clear_inputs();
                            app.tabs.index = 2;
                            app.scheduler_list.renew_schedules();
                            app.scheduler_list.set_rows();
                            app.scheduler_list.init_index_table();
                            app.change_mode(AppMode::SchedulerList);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}
