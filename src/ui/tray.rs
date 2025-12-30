use std::{sync::mpsc::Sender, time::Duration};

use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuId, MenuItemBuilder},
};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{DispatchMessageW, MSG, PM_REMOVE, PeekMessageW, TranslateMessage};

use crate::{APP_ICON_BYTES, agent, ui};

pub fn init_tray_icon() -> TrayIcon {
    let icon_data = ui::utils::load_icon_from_bytes(APP_ICON_BYTES);

    let icon = Icon::from_rgba(icon_data.rgba, icon_data.width, icon_data.height)
        .expect("Invalid icon data");
    let tray_menu = build_tray_menu();

    let tray = TrayIconBuilder::new()
        .with_tooltip("Time Tracker")
        .with_icon(icon)
        .with_menu(Box::new(tray_menu))
        .build()
        .unwrap();
    tray
}

pub fn start_tray_listener(command_tx: Sender<agent::AgentCommand>) {
    let menu_event_receiver = MenuEvent::receiver();
    let mut quit: bool = false;

    #[cfg(not(target_os = "windows"))]
    {
        while !quit {
            handle_events(&command_tx, menu_event_receiver, &mut quit);
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    #[cfg(target_os = "windows")]
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while !quit {
            while PeekMessageW(&mut msg as *mut MSG, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            handle_events(&command_tx, menu_event_receiver, &mut quit);
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}

fn handle_events(
    command_tx: &Sender<agent::AgentCommand>,
    menu_event_receiver: &crossbeam_channel::Receiver<MenuEvent>,
    quit: &mut bool,
) {
    while let Ok(event) = menu_event_receiver.try_recv() {
        match event.id.0.as_str() {
            "quit" => {
                let _ = command_tx.send(agent::AgentCommand::Quit);
                *quit = true;
            }
            "ui" => {
                let _ = command_tx.send(agent::AgentCommand::ShowUI);
            }
            _ => eprintln!("Invalid menu item"),
        }
    }
}

fn build_tray_menu() -> Menu {
    let quit_menu_item = MenuItemBuilder::new()
        .text("Quit")
        .id(MenuId("quit".to_string()))
        .enabled(true)
        .build();

    let open_ui_item = MenuItemBuilder::new()
        .text("Open UI")
        .id(MenuId("ui".into()))
        .enabled(true)
        .build();

    Menu::with_items(&[&open_ui_item, &quit_menu_item]).unwrap()
}
