use std::sync::{Arc, mpsc::Sender};

use eframe::{NativeOptions, egui};
use egui::{Align, Align2, Context, IconData, Layout, MenuBar, Order, ViewportBuilder, Window};
use rusqlite::Connection;

use crate::agent;

fn load_icon(path: &str) -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        (image.into_raw(), width, height)
    };
    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

pub fn run_ui(agent_tx: Sender<agent::AgentCommand>) {
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
                tasks: agent::tasks::get_all_tasks(&db_connection).unwrap(),
                show_new_task_dialog: false,
                new_task: agent::tasks::Task {
                    _id: 0,
                    name: "".to_string(),
                    description: "".to_string(),
                },
                db_connection,
            }))
        }),
    )
    .unwrap();
}

struct MyApp {
    agent_tx: Sender<agent::AgentCommand>,
    tasks: Vec<agent::tasks::Task>,
    show_new_task_dialog: bool,
    new_task: agent::tasks::Task,
    db_connection: Connection,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    if ui.button("Add Task").clicked() {
                        self.show_new_task_dialog = true;
                    }
                });
            });

            ui.group(|ui| {
                ui.heading("Tasks");
                for task in &mut self.tasks {
                    ui.group(|ui| {
                        ui.take_available_width();
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(&task.name);
                                ui.text_edit_multiline(&mut task.description);
                            });

                            ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                                if ui.button("Start").clicked() {
                                    self.agent_tx
                                        .send(agent::AgentCommand::StartTask(task.name.clone()))
                                        .unwrap();
                                }
                                if ui.button("Stop").clicked() {
                                    self.agent_tx.send(agent::AgentCommand::StopTask).unwrap();
                                }
                            });
                        });
                    });
                }
            });
        });

        if self.show_new_task_dialog {
            Window::new("New Task")
                .collapsible(false)
                .fixed_size([400.0, 100.0])
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .order(Order::Foreground)
                .show(ctx, |ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.new_task.name);
                    ui.label("Description:");
                    ui.text_edit_multiline(&mut self.new_task.description);
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                            if ui.button("Cancel").clicked() {
                                self.show_new_task_dialog = false;
                            }

                            if ui.button("Add").clicked() {
                                match agent::tasks::add_new_task(
                                    &self.db_connection,
                                    &self.new_task,
                                ) {
                                    Ok(_) => {
                                        display_message(
                                            ctx,
                                            "Successful",
                                            "Successfully added new task",
                                            &["OK"],
                                        );
                                        self.tasks =
                                            agent::tasks::get_all_tasks(&self.db_connection)
                                                .unwrap();
                                    }
                                    Err(e) => {
                                        display_message(ctx, "Error", &format!("{:?}", e), &[]);
                                    }
                                }
                                self.show_new_task_dialog = false;
                            }
                        });
                    });
                });
        } else {
            self.new_task = agent::tasks::Task {
                _id: 0,
                name: "".to_string(),
                description: "".to_string(),
            };
        }
    }
}

fn display_message(ctx: &Context, title: &str, message: &str, buttons: &[&str]) -> isize {
    Window::new(title)
        .collapsible(false)
        .fixed_size([200.0, 50.0])
        .resizable(false)
        .order(Order::Foreground)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label(message);
            ui.separator();
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    for (i, button) in buttons.iter().enumerate() {
                        if ui.button(*button).clicked() {
                            return i as isize;
                        }
                    }

                    return -1;
                });
            })
        });
    -1
}
