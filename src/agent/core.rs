use std::sync::{Arc, mpsc};

use rusqlite::Connection;

use crate::{agent, config, storage, ui};

struct AgentState {
    db_connection: Connection,

    session: agent::sessions::Session,
    stop_watch: agent::time::StopWatch,
    task_in_progress: bool,

    last_user_activity_time_stamp: chrono::DateTime<chrono::Utc>,
    user_state: ui::UserState,
}

impl AgentState {
    fn new(db_connection: Connection) -> Self {
        AgentState {
            db_connection,

            session: agent::sessions::Session::default(),
            stop_watch: agent::time::StopWatch::new(),
            task_in_progress: false,

            last_user_activity_time_stamp: chrono::Utc::now(),
            user_state: ui::UserState::Active,
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
    AddTask {
        task: agent::tasks::Task,
    },
    UserActivity {
        time_stamp: chrono::DateTime<chrono::Utc>,
    },
    RequestTaskList,
    Quit,
    RequestElapsedTime,
    ShowUI,
}

pub fn start_agent(
    command_rx: mpsc::Receiver<AgentCommand>,
    window_tx: crossbeam_channel::Sender<ui::UIEvent>,
    tray_tx: mpsc::Sender<ui::UIEvent>,
    ui_control_tx: mpsc::Sender<ui::UIControl>,
    settings: Arc<config::settings::Settings>,
) {
    let db_connection = storage::init_db(settings.clone()).unwrap();
    let mut agent_state = AgentState::new(db_connection);
    let mut running = true;

    if settings.open_ui_at_start_up {
        let _ = ui_control_tx.send(ui::UIControl::Show);
    }

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
                    window_tx.send(ui::UIEvent::TaskList { task_list }).unwrap();
                }
                AgentCommand::RequestElapsedTime => window_tx
                    .send(ui::UIEvent::ElapsedTime {
                        elapsed: agent_state.stop_watch.elapsed(),
                    })
                    .unwrap(),
                AgentCommand::Quit => {
                    let _ = window_tx.send(ui::UIEvent::Quit);
                    let _ = tray_tx.send(ui::UIEvent::Quit);
                    let _ = ui_control_tx.send(ui::UIControl::Quit);
                    running = false;
                }
                AgentCommand::ShowUI => {
                    let _ = ui_control_tx.send(ui::UIControl::Show);
                }
                AgentCommand::UserActivity { time_stamp } => {
                    agent_state.user_state = ui::UserState::Active;
                    agent_state.last_user_activity_time_stamp = time_stamp;
                    let _ = window_tx.send(ui::UIEvent::UserState {
                        state: agent_state.user_state,
                    });
                    let _ = tray_tx.send(ui::UIEvent::UserState {
                        state: agent_state.user_state,
                    });

                    if agent_state.task_in_progress {
                        agent_state.stop_watch.start();
                    }
                }
            }
        }

        let idle_after = agent_state.last_user_activity_time_stamp
            + chrono::Duration::seconds(settings.active_timeout_seconds.try_into().unwrap());
        let now = chrono::Utc::now();

        if agent_state.user_state == ui::UserState::Active {
            if now >= idle_after {
                agent_state.user_state = ui::UserState::Idle;
                let _ = window_tx.send(ui::UIEvent::UserState {
                    state: agent_state.user_state,
                });
                let _ = tray_tx.send(ui::UIEvent::UserState {
                    state: agent_state.user_state,
                });
                agent_state.stop_watch.stop();
                let _ = window_tx.send(ui::UIEvent::Repaint { time_out: 0 });
            };
        }
    }
}
