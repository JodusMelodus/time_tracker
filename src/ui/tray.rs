use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuId, MenuItemBuilder},
};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{DispatchMessageW, MSG, PM_REMOVE, PeekMessageW, TranslateMessage};

use crate::{ACTIVE_ICON_BYTES, IDLE_ICON_BYTES, agent, ui};

pub struct Tray {
    tray: TrayIcon,
    active_icon: Icon,
    idle_icon: Icon,
    command_tx: Sender<agent::AgentCommand>,
    event_rx: Receiver<ui::UIEvent>,
    quit: bool,
    user_state: ui::UserState,
}

impl Tray {
    pub fn new(command_tx: Sender<agent::AgentCommand>, tray_rx: Receiver<ui::UIEvent>) -> Self {
        let active_icon_data = ui::utils::load_icon_from_bytes(ACTIVE_ICON_BYTES);
        let idle_icon_data = ui::utils::load_icon_from_bytes(IDLE_ICON_BYTES);

        let active_icon = Icon::from_rgba(
            active_icon_data.rgba,
            active_icon_data.width,
            active_icon_data.height,
        )
        .expect("Invalid active icon data");

        let idle_icon = Icon::from_rgba(
            idle_icon_data.rgba,
            idle_icon_data.width,
            idle_icon_data.height,
        )
        .expect("Invalid idle icon data");

        let tray_menu = build_tray_menu();

        let tray = TrayIconBuilder::new()
            .with_tooltip("Time Tracker")
            .with_icon(active_icon.clone())
            .with_menu(Box::new(tray_menu))
            .build()
            .unwrap();

        let quit = false;
        let user_state = ui::UserState::Active;

        Self {
            tray,
            active_icon,
            idle_icon,
            command_tx,
            event_rx: tray_rx,
            quit,
            user_state,
        }
    }

    pub fn start_tray_listener(&mut self) {
        let menu_event_receiver = MenuEvent::receiver();

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
            while !self.quit {
                while PeekMessageW(&mut msg as *mut MSG, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0
                {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

                self.handle_events(menu_event_receiver);

                let _ = match self.user_state {
                    ui::UserState::Active => self.tray.set_icon(Some(self.active_icon.clone())),
                    ui::UserState::Idle => self.tray.set_icon(Some(self.idle_icon.clone())),
                };

                std::thread::sleep(Duration::from_millis(50));
            }
        }
    }

    fn handle_events(&mut self, menu_event_receiver: &crossbeam_channel::Receiver<MenuEvent>) {
        while let Ok(event) = menu_event_receiver.try_recv() {
            match event.id.0.as_str() {
                "quit" => {
                    let _ = self.command_tx.send(agent::AgentCommand::Quit);
                    self.quit = true;
                }
                "ui" => {
                    let _ = self.command_tx.send(agent::AgentCommand::ShowUI);
                }
                _ => eprintln!("Invalid menu item"),
            }
        }

        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                ui::UIEvent::UserState { state } => {
                    self.user_state = state;
                }
                _ => (),
            }
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
