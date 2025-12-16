mod agent;
mod storage;
mod ui;

fn main() {
    std::fs::create_dir_all("data").unwrap();
    let _conn = storage::sqlite::init_db().unwrap();
    println!("SQLite databse initialized!");

    agent::input::start_input_listener();
    println!("Input listener running. Press Ctrl+C to exit.");

    ui::windows::run_ui();
}
