use std::sync::mpsc::Sender;

use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuId, MenuItemBuilder},
};

use crate::agent;

pub struct Tray {
    agent_tx: Sender<agent::AgentCommand>,
    _tray_icon: TrayIcon,
}

impl Tray {
    pub fn init_tray_icon(agent_tx: Sender<agent::AgentCommand>) -> Tray {
        let icon = Icon::from_path("icon.ico", Some((128, 128))).unwrap();

        let quit_menu_item = MenuItemBuilder::new()
            .text("Quit")
            .id(MenuId("quit".to_string()))
            .enabled(true)
            .build();

        let tray_menu = Menu::with_items(&[&quit_menu_item]).unwrap();

        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Time Tracker")
            .with_icon(icon)
            .build()
            .unwrap();

        Tray {
            agent_tx,
            _tray_icon,
        }
    }

    pub fn start_tray_icon(self) {
        let menu_event_receiver = MenuEvent::receiver();

        loop {
            while let Ok(event) = menu_event_receiver.try_recv() {
                match event.id.0.as_str() {
                    "quit" => {
                        self.agent_tx.send(agent::AgentCommand::Quit).unwrap();
                    }
                    _ => eprintln!("Unknown menu item"),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
