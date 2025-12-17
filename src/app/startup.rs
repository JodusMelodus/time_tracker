use std::sync::mpsc;

use crate::{agent, app::types::AppState, storage, ui};

pub fn start() {
    // Ensure local database is created
    std::fs::create_dir_all("data").unwrap();
    let _conn = storage::sqlite::init_db().unwrap();
    println!("SQLite databse initialized!");

    // Start local agent to continuesly get events
    
    let _app_state = AppState {
        _tray_icon: ui::tray::init_tray_icon(),
    };
    
    let (agent_tx, agent_rx) = mpsc::channel();
    
    agent::input::start_input_listener(agent_tx.clone());
    println!("Input listener running. Press Ctrl+C to exit.");
    // agent::start_agent(agent_rx);
        
    // Initialize tray icon
    ui::tray::start_tray_icon();

    // Open ui
    ui::windows::run_ui(agent_tx, agent_rx);
}
