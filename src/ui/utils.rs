use std::time::Duration;

use egui::IconData;

pub fn load_icon_from_bytes(bytes: &[u8]) -> IconData {
    let image = image::load_from_memory(bytes)
        .expect("Failed to load icon bytes")
        .into_rgba8();
    let (w, h) = image.dimensions();

    IconData {
        rgba: image.into_raw(),
        width: w,
        height: h,
    }
}

pub fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();

    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
