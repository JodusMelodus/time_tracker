use std::{
    sync::{Arc, mpsc},
    thread,
};

use crate::{agent, config, ui};

pub fn start() {
    let (command_tx, command_rx) = mpsc::channel();
    let (event_tx, event_rx) = crossbeam_channel::unbounded();
    let (ui_control_tx, ui_control_rx) = mpsc::channel();

    let settings = Arc::new(config::Settings::load());
    let agent_settings = settings.clone();
    let agent_event_tx = event_tx.clone();
    let tray_command = command_tx.clone();

    let agent_thread = thread::Builder::new()
        .name("agent-worker".into())
        .spawn(move || {
            agent::start_input_listener(agent_event_tx.clone());
            agent::start_agent(command_rx, agent_event_tx, ui_control_tx, agent_settings);
        })
        .expect("Failed to spawn agent-worker thread");

    let tray_thread = thread::Builder::new()
        .name("tray-menu".to_string())
        .spawn(move || {
            let _tray = ui::init_tray_icon();
            ui::start_tray_listener(tray_command)
        })
        .unwrap();

    if settings.open_ui_at_start_up {
        ui::run_ui(command_tx.clone(), event_rx.clone(), settings.clone());
    }

    loop {
        if let Ok(event) = ui_control_rx.try_recv() {
            match event {
                ui::UIControl::Show => {
                    ui::run_ui(command_tx.clone(), event_rx.clone(), settings.clone())
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
