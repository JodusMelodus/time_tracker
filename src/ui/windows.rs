use std::sync::{Arc, mpsc::Sender};

use eframe::{Frame, NativeOptions, egui};
use egui::{Align, IconData, Layout, MenuBar, ViewportBuilder, menu};
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
    let conn = Connection::open("data/sessions.db").unwrap();

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
                tasks: agent::tasks::get_all_tasks(&conn).unwrap(),
            }))
        }),
    )
    .unwrap();
}

struct MyApp {
    agent_tx: Sender<agent::AgentCommand>,
    tasks: Vec<agent::tasks::Task>,
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
    }
}
