#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod agent;
mod app;
mod config;
mod storage;
mod ui;
mod utils;

pub static APP_ICON_BYTES: &[u8] = include_bytes!("../assets/icon.ico");
pub static DB_SCHEMA: &str = include_str!("../assets/schema.sql");

fn main() {
    app::startup::start();
}
