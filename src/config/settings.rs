use std::{fs, path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub auto_sync_interval_seconds: u64,
    pub active_timeout_seconds: u64,
}

impl Settings {
    pub fn load() -> Self {
        let path = settings_path();

        let settings = if let Ok(data) = fs::read_to_string(path) {
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

    pub fn save(&self) -> std::io::Result<()> {
        let path = settings_path();
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, json)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_sync_interval_seconds: 30,
            active_timeout_seconds: 15,
        }
    }
}

fn settings_path() -> path::PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or(std::env::current_dir().unwrap())
        .join("time-tracker");
    std::fs::create_dir_all(&dir).ok();
    dir.join("settings.json")
}
