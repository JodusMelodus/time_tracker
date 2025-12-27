use std::thread;

use chrono::Utc;
use crossbeam_channel::Sender;
use rdev::{Event, listen};

use crate::ui;

pub fn start_input_listener(event_tx: Sender<ui::UIEvent>) {
    thread::Builder::new()
        .name("agent-listener".to_string())
        .spawn(move || {
            let _ = listen(move |_event: Event| {
                let _ = event_tx.send(ui::UIEvent::UserActivity {
                    time_stamp: Utc::now(),
                });
            });
        })
        .expect("Failed to spawn agent-listener thread");
}
