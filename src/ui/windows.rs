use std::sync::{
    Arc,
    mpsc::{Receiver, Sender},
};

use eframe::{NativeOptions, egui};
use egui::{
    Align, Align2, CentralPanel, Color32, Context, IconData, Layout, MenuBar, Order, ScrollArea,
    Slider, TopBottomPanel, ViewportBuilder, ViewportCommand, Window, panel::TopBottomSide,
};
use rusqlite::Connection;

use crate::{agent, app};

fn load_icon(path: &str) -> IconData {
    let (rgba, width, height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        (image.into_raw(), width, height)
    };
    IconData {
        rgba,
        width,
        height,
    }
}

pub fn run_ui(agent_tx: Sender<agent::AgentCommand>, agent_rx: Receiver<agent::AgentCommand>) {
    let db_connection = Connection::open("data/sessions.db").unwrap();
    let icon = load_icon("icon.ico");
    let mut options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size([250.0, 500.0])
            .with_max_inner_size([500.0, 750.0]),
        ..Default::default()
    };

    options.viewport.icon = Some(Arc::new(icon));
    eframe::run_native(
        "Time Tracker",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                agent_tx,
                agent_rx,
                tasks: agent::tasks::get_all_tasks(&db_connection).unwrap(),
                show_new_task_dialog: false,
                new_task: agent::tasks::Task::default(),
                db_connection,
                last_user_activity_time_stamp: chrono::Utc::now(),
                user_state: app::types::UserState::Active,
            }))
        }),
    )
    .unwrap();
}

struct MyApp {
    agent_tx: Sender<agent::AgentCommand>,
    agent_rx: Receiver<agent::AgentCommand>,
    tasks: Vec<agent::tasks::Task>,
    show_new_task_dialog: bool,
    new_task: agent::tasks::Task,
    db_connection: Connection,
    user_state: app::types::UserState,
    last_user_activity_time_stamp: chrono::DateTime<chrono::Utc>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        while let Ok(command) = self.agent_rx.try_recv() {
            match command {
                agent::AgentCommand::UserActive { time_stamp } => {
                    self.user_state = app::types::UserState::Active;
                    self.last_user_activity_time_stamp = time_stamp;
                }
                _ => {}
            }
        }

        let idle_after = self.last_user_activity_time_stamp + chrono::Duration::seconds(5);
        let now = chrono::Utc::now();

        if self.user_state == app::types::UserState::Active {
            if now >= idle_after {
                self.user_state = app::types::UserState::Idle;
                ctx.request_repaint();
            } else {
                let remaining = idle_after - now;
                ctx.request_repaint_after(std::time::Duration::from_secs(
                    remaining.num_seconds() as u64
                ));
            }
        }

        TopBottomPanel::new(TopBottomSide::Top, "Menu Bar").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                    }
                });
                ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    if ui.button("Add Task").clicked() {
                        self.show_new_task_dialog = true;
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Tasks");
                ScrollArea::vertical().show(ui, |ui| {
                    for task in &mut self.tasks {
                        ui.group(|ui| {
                            ui.take_available_width();
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(&task.t_name);
                                    ui.text_edit_multiline(&mut task.t_description);
                                });

                                ui.vertical(|ui| {
                                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                        match task.t_priority {
                                            0 => ui.colored_label(
                                                Color32::DARK_GREEN,
                                                agent::tasks::PRIORITY_LEVELS[task.t_priority],
                                            ),
                                            1 => ui.colored_label(
                                                Color32::YELLOW,
                                                agent::tasks::PRIORITY_LEVELS[task.t_priority],
                                            ),
                                            2 => ui.colored_label(
                                                Color32::RED,
                                                agent::tasks::PRIORITY_LEVELS[task.t_priority],
                                            ),
                                            _ => ui.label(
                                                agent::tasks::PRIORITY_LEVELS[task.t_priority],
                                            ),
                                        };
                                    });

                                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                                        // match task.in_progress {
                                        //     true => {
                                        //         if ui.button("Stop").clicked() {
                                        //             self.agent_tx
                                        //                 .send(agent::AgentCommand::StopTask)
                                        //                 .unwrap();
                                        //         }
                                        //     }
                                        //     false => {
                                        //         if ui.button("Start").clicked() {
                                        //             self.agent_tx
                                        //                 .send(agent::AgentCommand::StartTask {
                                        //                     name: task.t_name.clone(),
                                        //                 })
                                        //                 .unwrap();
                                        //         }
                                        //     }
                                        // }

                                        // TODO: fix start stop
                                    });
                                });
                            });
                        });
                    }
                });
            });
        });

        TopBottomPanel::new(TopBottomSide::Bottom, "Status Bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    match self.user_state {
                        app::types::UserState::Active => {
                            ui.colored_label(Color32::DARK_GREEN, "Active")
                        }
                        app::types::UserState::Idle => ui.colored_label(Color32::DARK_GRAY, "Idle"),
                    };
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Max), |_ui| {});
            });
        });

        if self.show_new_task_dialog {
            self.new_task_dialog(ctx);
        } else {
            self.new_task = agent::tasks::Task::default();
        }
    }
}

impl MyApp {
    fn new_task_dialog(&mut self, ctx: &Context) {
        Window::new("New Task")
            .collapsible(false)
            .fixed_size([400.0, 100.0])
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .order(Order::Foreground)
            .show(ctx, |ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.new_task.t_name);
                ui.label("Description:");
                ui.text_edit_multiline(&mut self.new_task.t_description);

                ui.horizontal(|ui| {
                    ui.label("Priority");
                    let level = self.new_task.t_priority;
                    ui.add(
                        Slider::new(&mut self.new_task.t_priority, 0..=2)
                            .step_by(0.33)
                            .text(agent::tasks::PRIORITY_LEVELS[level])
                            .show_value(false),
                    );
                });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_new_task_dialog = false;
                        }

                        if ui.button("Add").clicked() {
                            match agent::tasks::add_new_task(&self.db_connection, &self.new_task) {
                                Ok(_) => {
                                    println!("Add new task");
                                    self.tasks =
                                        agent::tasks::get_all_tasks(&self.db_connection).unwrap();
                                }
                                Err(e) => {
                                    println!("Failed to add task: {}", e);
                                }
                            }
                            self.show_new_task_dialog = false;
                        }
                    });
                });
            });
    }
}
