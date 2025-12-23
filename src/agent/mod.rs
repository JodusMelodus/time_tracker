use std::{
    sync::mpsc::{Receiver, Sender},
    time::Instant,
};

use rusqlite::Connection;

use crate::{agent, storage, ui};

pub mod input;
pub mod sessions;
pub mod tasks;

#[derive(PartialEq)]
pub enum UserState {
    Idle,
    Active,
}

pub struct AgentState {
    pub db_connection: Connection,

    pub session: agent::sessions::Session,
    pub start_time: Instant,
    pub task_in_progress: bool,
}

impl AgentState {
    pub fn new(db_connection: Connection) -> Self {
        AgentState {
            db_connection,

            session: agent::sessions::Session::default(),
            start_time: Instant::now(),
            task_in_progress: false,
        }
    }
}

pub enum AgentCommand {
    StartSession {
        id: i64,
    },
    EndSession {
        comment: String,
    },
    UserActive {
        time_stamp: chrono::DateTime<chrono::Utc>,
    },
    AddTask {
        task: agent::tasks::Task,
    },
    RequestTaskList,
    RequestTaskState,
    Quit,
}

pub fn start_agent(command_rx: Receiver<AgentCommand>, event_tx: Sender<ui::UIEvent>) {
    let db_connection = storage::sqlite::init_db().unwrap();
    println!("SQLite databse initialized!");
    let mut agent_state = agent::AgentState::new(db_connection);

    loop {
        while let Ok(event) = command_rx.try_recv() {
            match event {
                AgentCommand::StartSession { id } => {
                    agent_state.task_in_progress = true;
                    agent_state.session = agent::sessions::Session::default();
                    agent_state.session.s_task = id;
                }
                AgentCommand::EndSession { comment } => {
                    agent_state.task_in_progress = false;
                    let end_time = Instant::now();
                    agent_state.session.s_duration = (end_time - agent_state.start_time).as_secs();
                    agent_state.session.s_comment = comment;
                    agent::sessions::save_session(&agent_state.db_connection, &agent_state.session)
                        .unwrap();
                }
                AgentCommand::AddTask { task } => {
                    agent::tasks::add_new_task(&agent_state.db_connection, &task).unwrap();
                }
                AgentCommand::RequestTaskList => {
                    let task_list =
                        agent::tasks::get_all_tasks(&agent_state.db_connection).unwrap();
                    event_tx.send(ui::UIEvent::TaskList { task_list }).unwrap();
                }
                AgentCommand::RequestTaskState => {
                    let state = agent_state.task_in_progress;
                    event_tx.send(ui::UIEvent::ProgressState { state }).unwrap();
                }
                _ => (),
            }
        }
    }
}
