use std::{sync::mpsc::Sender, time::Duration};

use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuId, MenuItemBuilder},
};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{DispatchMessageW, MSG, PM_REMOVE, PeekMessageW, TranslateMessage};

use crate::agent::{self};

pub fn init_tray_icon() -> TrayIcon {
    let icon = Icon::from_path("icon.ico", Some((128, 128))).unwrap();
    let tray_menu = build_tray_menu();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Time Tracker")
        .with_icon(icon)
        .build()
        .unwrap();
    tray
}

pub fn start_tray_listener(command_tx: Sender<agent::AgentCommand>) {
    let menu_event_receiver = MenuEvent::receiver();
    #[cfg(not(target_os = "windows"))]
    {
        let quit = false;
        while !quit {
            while let Ok(event) = menu_event_receiver.try_recv() {
                match event.id.0.as_str() {
                    "quit" => command_tx.send(agent::AgentCommand::Quit).unwrap(),
                    _ => eprintln!("Unknown menu item"),
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    #[cfg(target_os = "windows")]
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        loop {
            while PeekMessageW(&mut msg as *mut MSG, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            while let Ok(event) = menu_event_receiver.try_recv() {
                match event.id.0.as_str() {
                    "quit" => {
                        let _ = command_tx.send(agent::AgentCommand::Quit);
                        return;
                    }
                    _ => eprintln!("Unknown menu item"),
                }
            }

            std::thread::sleep(Duration::from_millis(50));
        }
    }
}

fn build_tray_menu() -> Menu {
    let quit_menu_item = MenuItemBuilder::new()
        .text("Quit")
        .id(MenuId("quit".to_string()))
        .enabled(true)
        .build();

    Menu::with_items(&[&quit_menu_item]).unwrap()
}
