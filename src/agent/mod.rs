use std::sync::mpsc::{Receiver, Sender};

use rusqlite::Connection;

use crate::{agent, storage, ui, utils};

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
    pub stop_watch: utils::time::StopWatch,
    pub task_in_progress: bool,
}

impl AgentState {
    pub fn new(db_connection: Connection) -> Self {
        AgentState {
            db_connection,

            session: agent::sessions::Session::default(),
            stop_watch: utils::time::StopWatch::new(),
            task_in_progress: false,
        }
    }
}

pub enum AgentCommand {
    StartSession { id: i64 },
    EndSession { comment: String },
    AddTask { task: agent::tasks::Task },
    RequestTaskList,
    RequestTaskState,
    Quit,
    UpdateStopWatch { running: bool },
    ElapsedTime,
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
                    agent_state.stop_watch.start();
                    agent_state.session.s_task = id;
                }
                AgentCommand::EndSession { comment } => {
                    agent_state.task_in_progress = false;
                    agent_state.stop_watch.stop();
                    let elapsed_time = agent_state.stop_watch.elapsed();
                    agent_state.stop_watch.reset();

                    agent_state.session.s_comment = comment;
                    agent_state.session.s_duration = elapsed_time.as_secs();
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
                AgentCommand::UpdateStopWatch { running } => {
                    if agent_state.task_in_progress {
                        match running {
                            true => agent_state.stop_watch.start(),
                            false => agent_state.stop_watch.stop(),
                        }
                    }
                }
                AgentCommand::ElapsedTime => event_tx
                    .send(ui::UIEvent::ElapsedTime {
                        elapsed: agent_state.stop_watch.elapsed(),
                    })
                    .unwrap(),
                AgentCommand::Quit => todo!("Do this"),
            }
        }
    }
}
