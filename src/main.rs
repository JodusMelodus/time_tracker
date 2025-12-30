#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub static APP_ICON_BYTES: &[u8] = include_bytes!("../assets/icon.ico");
pub static DB_SCHEMA: &str = include_str!("../assets/schema.sql");

pub mod agent;
pub mod app;
pub mod config;
pub mod storage;
pub mod ui;

fn main() {
    app::start();
}
