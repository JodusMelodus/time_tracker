use crate::{agent, app::state::AppState, storage, ui};

pub fn start() {
    // Ensure local database is created
    std::fs::create_dir_all("data").unwrap();
    let _conn = storage::sqlite::init_db().unwrap();
    println!("SQLite databse initialized!");

    // Start local agent to continuesly get events
    agent::input::start_input_listener();
    println!("Input listener running. Press Ctrl+C to exit.");

    let _app_state = AppState {
        tray_icon: ui::tray::init_tray_icon(),
    };
    
    // Initialize tray icon
    ui::tray::start_tray_icon();

    // Open ui
    ui::windows::run_ui();
}
