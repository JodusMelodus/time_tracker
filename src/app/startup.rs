use std::sync::mpsc;

use crate::{agent, app, storage, ui};

pub fn start() {
    // Ensure local database is created
    std::fs::create_dir_all("data").unwrap();
    let db_connection = storage::sqlite::init_db().unwrap();
    println!("SQLite databse initialized!");

    let app_state = app::types::AppState::new(db_connection);
    let (command_tx, command_rx) = mpsc::channel();
    let (event_tx, event_rx) = mpsc::channel();

    // Start local agent to continuesly get events
    agent::input::start_input_listener(command_tx.clone());
    println!("Input listener running. Press Ctrl+C to exit.");
    agent::start_agent(command_rx, event_tx, app_state);

    // Initialize tray icon
    ui::tray::start_tray_icon();

    // Open ui
    ui::window::run_ui(command_tx, event_rx);
}
