use crate::app::state::AppState;

mod agent;
mod app;
mod storage;
mod ui;

fn main() {
    // Ensure local database is created
    std::fs::create_dir_all("data").unwrap();
    let _conn = storage::sqlite::init_db().unwrap();
    println!("SQLite databse initialized!");

    // Start local agent to continuesly get events
    agent::input::start_input_listener();
    println!("Input listener running. Press Ctrl+C to exit.");

    // Initialize tray icon

    let _app_state = AppState {
        tray_icon: ui::tray::init_tray_icon(),
    };

    // Open ui
    ui::windows::run_ui();
}
