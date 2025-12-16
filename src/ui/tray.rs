use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem},
};

pub fn init_tray_icon() -> TrayIcon {
    let icon = Icon::from_path("icon.ico", Some((128, 128))).unwrap();
    let quit = MenuItem::new("Quit", true, None);
    let tray_menu = Menu::with_items(&[&quit]).unwrap();

    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Time Tracker")
        .with_icon(icon)
        .build()
        .unwrap()
}
