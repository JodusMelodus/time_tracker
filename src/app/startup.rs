use std::{
    sync::{Arc, mpsc},
    thread,
};

use crate::{agent, config, ui};

pub fn start() {
    std::fs::create_dir_all("data").unwrap();

    let (command_tx, command_rx) = mpsc::channel();
    let (event_tx, event_rx) = crossbeam_channel::unbounded();
    let settings = Arc::new(config::settings::Settings::load());
    let agent_settings = settings.clone();
    let agent_event_tx = event_tx.clone();
    let tray_command = command_tx.clone();

    let agent_thread = thread::Builder::new()
        .name("agent-worker".into())
        .spawn(move || {
            agent::input::start_input_listener(agent_event_tx.clone());
            agent::start_agent(command_rx, agent_event_tx, agent_settings);
        })
        .expect("Failed to spawn agent-worker thread");

    let tray_thread = std::thread::Builder::new()
        .name("tray-menu".to_string())
        .spawn(move || {
            let _tray = ui::tray::init_tray_icon();
            ui::tray::start_tray_listener(tray_command)
        })
        .unwrap();

    ui::window::run_ui(command_tx, event_rx, settings.clone());

    let _ = tray_thread.join();
    let _ = agent_thread.join();
}
