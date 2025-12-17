use tray_icon::TrayIcon;

pub struct AppState {
    pub _tray_icon: TrayIcon,
}

#[derive(PartialEq)]
pub enum UserState {
    Idle,
    Active,
}
