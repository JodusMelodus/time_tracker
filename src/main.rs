#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub static APP_ICON_BYTES: &[u8] = include_bytes!("../assets/icon.ico");
pub static DB_SCHEMA: &str = include_str!("../assets/schema.sql");
pub static ACTIVE_ICON_BYTES: &[u8] = include_bytes!("../assets/active.png");
pub static IDLE_ICON_BYTES: &[u8] = include_bytes!("../assets/idle.png");

pub mod agent;
pub mod app;
pub mod config;
pub mod storage;
pub mod ui;

fn main() {
    app::start();
}
