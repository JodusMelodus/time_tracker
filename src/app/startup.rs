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
        let agent_command_tx = command_tx.clone();
        thread::Builder::new()
            .name("agent-worker".to_string())
            .spawn(move || {
                println!("Starting agent-worker thread...");
                agent::input::start_input_listener(agent_command_tx.clone());
                agent::start_agent(command_rx, event_tx);
            })
            .expect("Failed to spawn agent-worker thread");
    }

    // Initialize tray icon
    {
        let tray_command_tx = command_tx.clone();
        thread::Builder::new()
            .name("tray".to_string())
            .spawn(move || {
                println!("Starting tray thread...");
                let tray = ui::tray::Tray::init_tray_icon(tray_command_tx.clone());
                tray.start_tray_icon();
            })
            .expect("Failed to spawn tray thread");
    }

    // Open ui
    ui::window::run_ui(command_tx, event_rx);
}
