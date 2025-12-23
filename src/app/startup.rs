use std::{sync::mpsc, thread};

use crate::{
    agent,
    ui::{self},
};

pub fn start() {
    std::fs::create_dir_all("data").unwrap();

    let (command_tx, command_rx) = mpsc::channel();
    let (event_tx, event_rx) = mpsc::channel();

    // Start local agent
    {
        thread::Builder::new()
            .name("agent-worker".to_string())
            .spawn(move || {
                println!("Starting agent-worker thread...");
                agent::input::start_input_listener(event_tx.clone());
                agent::start_agent(command_rx, event_tx);
            })
            .expect("Failed to spawn agent-worker thread");
    }

    // Initialize tray icon
    println!("Starting tray thread...");
    let _tray = ui::tray::init_tray_icon(command_tx.clone());

    // Open ui
    ui::window::run_ui(command_tx, event_rx);
}
