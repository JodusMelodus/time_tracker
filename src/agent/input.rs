use std::sync::mpsc::Sender;

use rdev::{Event, listen};

use crate::agent::AgentCommand;

pub fn start_input_listener(tx: Sender<AgentCommand>) {
    std::thread::spawn(move || {
        let _ = listen(move |_event: Event| {
            let _ = tx.send(AgentCommand::UserActive {
                time_stamp: chrono::Utc::now(),
            });
        });
    });
}
