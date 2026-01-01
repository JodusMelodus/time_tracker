use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::agent;

pub enum UIEvent {
    TaskList { task_list: Vec<agent::tasks::Task> },
    UserActivity { time_stamp: DateTime<Utc> },
    ElapsedTime { elapsed: Duration },
    UserState { state: UserState },
    Quit,
}

pub enum UIControl {
    Show,
    Quit,
}

#[derive(PartialEq)]
pub enum UserState {
    Idle,
    Active,
}
