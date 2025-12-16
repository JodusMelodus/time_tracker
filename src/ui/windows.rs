use eframe::egui;

pub fn run_ui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Time Tracker",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp {}))),
    )
    .unwrap();
}

struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasks");
            if ui.button("Start Task").clicked() {
                println!("Start task clicked");
            }
            if ui.button("Stop Task").clicked() {
                println!("Stop task clicked");
            }
        });
    }
}
