use std::{sync::mpsc::Sender, thread};

use rdev::{Event, listen};

use crate::agent;

pub fn start_input_listener(tx: Sender<agent::AgentCommand>) {
    thread::Builder::new()
        .name("agent-listener".to_string())
        .spawn(move || {
            let _ = listen(move |_event: Event| {
                let _ = tx.send(agent::AgentCommand::UserActive {
                    time_stamp: chrono::Utc::now(),
                });
            });
        })
        .expect("Failed to spawn agent-listener thread");
}
