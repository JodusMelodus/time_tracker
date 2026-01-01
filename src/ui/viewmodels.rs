use std::time::Duration;

use crate::agent;

pub enum UIEvent {
    TaskList { task_list: Vec<agent::tasks::Task> },
    ElapsedTime { elapsed: Duration },
    UserState { state: UserState },
    Repaint { time_out: u64 },
    Quit,
}

pub enum UIControl {
    Show,
    Quit,
}

#[derive(PartialEq, Clone, Copy)]
pub enum UserState {
    Idle,
    Active,
}
