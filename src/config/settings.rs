use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub auto_sync_interval_seconds: u64,
    pub active_timeout_seconds: u64,
    pub local_database_path: String,
    pub uid: String,
    pub open_ui_at_start_up: bool,
}

impl Settings {
    pub fn load() -> Self {
        let settings = if let Ok(data) = fs::read_to_string(settings_path()) {
            if let Ok(settings) = serde_json::from_str::<Self>(&data) {
                settings
            } else {
                Self::default()
            }
        } else {
            Self::default()
        };

        let _ = Self::save(&settings);
        settings
    }

    fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(settings_path(), json)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_sync_interval_seconds: 30,
            active_timeout_seconds: 15,
            local_database_path: local_database_path().to_string_lossy().to_string(),
            uid: Uuid::new_v4().to_string(),
            open_ui_at_start_up: true,
        }
    }
}

fn settings_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or(std::env::current_dir().unwrap())
        .join("time-tracker");
    std::fs::create_dir_all(&dir).ok();
    dir.join("settings.json")
}

fn local_database_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or(std::env::current_dir().unwrap())
        .join("time-tracker");
    std::fs::create_dir_all(&dir).ok();
    dir.join("sessions.db")
}
