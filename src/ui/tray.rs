use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuId, MenuItemBuilder},
};

pub fn init_tray_icon() -> TrayIcon {
    let icon = Icon::from_path("icon.ico", Some((128, 128))).unwrap();

    let quit_menu_item = MenuItemBuilder::new()
        .text("Quit")
        .id(MenuId("quit".to_string()))
        .enabled(true)
        .build();

    let tray_menu = Menu::with_items(&[&quit_menu_item]).unwrap();

    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Time Tracker")
        .with_icon(icon)
        .build()
        .unwrap()
}

pub fn start_tray_icon() {
    println!("Starting tray icon");

    std::thread::spawn(move || {
        let menu_event_receiver = MenuEvent::receiver();

        loop {
            while let Ok(event) = menu_event_receiver.try_recv() {
                match event.id.0.as_str() {
                    "quit" => println!("Quit"),
                    _ => eprintln!("Unknown menu item"),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}
