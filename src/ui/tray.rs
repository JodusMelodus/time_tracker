use std::{sync::mpsc::Sender, time::Duration};

use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuId, MenuItemBuilder},
};

use crate::agent::{self, AgentCommand};

pub fn init_tray_icon(agent_tx: Sender<agent::AgentCommand>) -> TrayIcon {
    let icon = Icon::from_path("icon.ico", Some((128, 128))).unwrap();
    let tray_menu = build_tray_menu();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Time Tracker")
        .with_icon(icon)
        .build()
        .unwrap();

    std::thread::Builder::new()
        .name("tray-menu".to_string())
        .spawn(move || {
            let menu_event_receiver = MenuEvent::receiver();

            loop {
                while let Ok(event) = menu_event_receiver.try_recv() {
                    match event.id.0.as_str() {
                        "quit" => agent_tx.send(AgentCommand::Quit).unwrap(),
                        _ => eprintln!("Unknown menu item"),
                    }
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        })
        .unwrap();

    tray
}

fn build_tray_menu() -> Menu {
    let quit_menu_item = MenuItemBuilder::new()
        .text("Quit")
        .id(MenuId("quit".to_string()))
        .enabled(true)
        .build();

    Menu::with_items(&[&quit_menu_item]).unwrap()
}
