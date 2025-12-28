#![windows_subsystem = "windows"]

mod agent;
mod app;
mod config;
mod storage;
mod ui;
mod utils;

fn main() {
    app::startup::start();
}
