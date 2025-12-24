use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::agent;

pub mod tray;
pub mod window;

pub enum UIEvent {
    TaskList { task_list: Vec<agent::tasks::Task> },
    ProgressState { state: bool },
    UserActivity { time_stamp: DateTime<Utc> },
    ElapsedTime { elapsed: Duration },
}
