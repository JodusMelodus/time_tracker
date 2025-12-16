use std::sync::{Arc, mpsc::Sender};

use eframe::egui;
use egui::IconData;

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
    let icon = load_icon("icon.ico");
    let mut options = eframe::NativeOptions::default();
    options.viewport.icon = Some(Arc::new(icon));

    eframe::run_native(
        "Time Tracker",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp { agent_tx }))),
    )
    .unwrap();
}

struct MyApp {
    agent_tx: Sender<agent::AgentCommand>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasks");
            if ui.button("Start Task").clicked() {
                self.agent_tx
                    .send(agent::AgentCommand::StartTask("test".to_string()))
                    .unwrap();
            }
            if ui.button("Stop Task").clicked() {
                self.agent_tx.send(agent::AgentCommand::StopTask).unwrap();
            }
        });
    }
}
