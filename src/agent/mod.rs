use std::sync::mpsc::Receiver;

pub mod input;
pub mod tasks;

pub enum AgentCommand {
    StartTask(String),
    StopTask,
}

pub fn start_agent(rx: Receiver<AgentCommand>) {
    std::thread::spawn(move || {
        loop {
            if let Ok(command) = rx.try_recv() {
                match command {
                    AgentCommand::StartTask(name) => println!("Started task: {}", name),
                    AgentCommand::StopTask => println!("Stoped task"),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
