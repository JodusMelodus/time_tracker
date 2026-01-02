use std::{
    sync::{Arc, mpsc},
    thread,
};

use crate::{agent, config, ui};

pub fn start() {
    let (command_tx, command_rx) = mpsc::channel();
    let (window_tx, window_rx) = crossbeam_channel::unbounded();
    let (tray_tx, tray_rx) = mpsc::channel();
    let (ui_control_tx, ui_control_rx) = mpsc::channel();

    let settings = Arc::new(config::Settings::load());

    let agent_settings = settings.clone();
    let agent_command_tx = command_tx.clone();
    let agent_thread = thread::Builder::new()
        .name("agent-worker".into())
        .spawn(move || {
            agent::start_input_listener(agent_command_tx);
            agent::start_agent(
                command_rx,
                window_tx,
                tray_tx,
                ui_control_tx,
                agent_settings,
            );
        })
        .expect("Failed to spawn agent-worker thread");

    let tray_command_tx = command_tx.clone();
    let tray_thread = thread::Builder::new()
        .name("tray-menu".to_string())
        .spawn(move || {
            let mut tray = ui::Tray::new(tray_command_tx, tray_rx);
            tray.start_tray_listener();
        })
        .unwrap();

    loop {
        if let Ok(event) = ui_control_rx.try_recv() {
            match event {
                ui::UIControl::Show => {
                    ui::run_ui(command_tx.clone(), window_rx.clone(), settings.clone())
                }
                ui::UIControl::Quit => {
                    break;
                }
            }
        }
    }

    let _ = tray_thread.join();
    let _ = agent_thread.join();
}
