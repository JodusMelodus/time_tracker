use std::sync::Arc;

use rusqlite::Connection;

use crate::{agent, config, storage, ui};

struct AgentState {
    db_connection: Connection,

    session: agent::sessions::Session,
    stop_watch: agent::time::StopWatch,
    task_in_progress: bool,
}

impl AgentState {
    fn new(db_connection: Connection) -> Self {
        AgentState {
            db_connection,

            session: agent::sessions::Session::default(),
            stop_watch: agent::time::StopWatch::new(),
            task_in_progress: false,
        }
    }
}

pub enum AgentCommand {
    StartSession { id: i64 },
    EndSession { comment: String },
    AddTask { task: agent::tasks::Task },
    UpdateStopWatch { running: bool },
    RequestTaskList,
    Quit,
    ElapsedTime,
    ShowUI,
}

pub fn start_agent(
    command_rx: std::sync::mpsc::Receiver<AgentCommand>,
    event_tx: crossbeam_channel::Sender<ui::viewmodels::UIEvent>,
    ui_control_tx: std::sync::mpsc::Sender<ui::viewmodels::UIControl>,
    settings: Arc<config::settings::Settings>,
) {
    let db_connection = storage::init_db(settings.clone()).unwrap();
    let mut agent_state = AgentState::new(db_connection);
    let mut running = true;

    while running {
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

                    agent_state.session.s_user = settings.uid.clone();
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
                    event_tx
                        .send(ui::viewmodels::UIEvent::TaskList { task_list })
                        .unwrap();
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
                    .send(ui::viewmodels::UIEvent::ElapsedTime {
                        elapsed: agent_state.stop_watch.elapsed(),
                    })
                    .unwrap(),
                AgentCommand::Quit => {
                    let _ = event_tx.send(ui::viewmodels::UIEvent::Quit);
                    let _ = ui_control_tx.send(ui::viewmodels::UIControl::Quit);
                    running = false;
                }
                AgentCommand::ShowUI => {
                    let _ = ui_control_tx.send(ui::viewmodels::UIControl::Show);
                }
            }
        }
    }
}
