use rusqlite::{Connection, Result};

#[derive(Clone)]
pub struct Task {
    pub t_id: i64,
    pub t_name: String,
    pub t_priority: usize,
}

pub const PRIORITY_LEVELS: &[&str] = &["Low", "Medium", "High"];

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut statement = conn.prepare("SELECT * FROM tasks")?;
    let task_iter = statement.query_map([], |row| {
        Ok(Task {
            t_id: row.get(0)?,
            t_name: row.get(1)?,
            t_priority: row.get(2)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

pub fn add_new_task(conn: &Connection, task: &Task) -> Result<usize> {
    conn.execute(
        "INSERT INTO tasks
            (t_name, t_priority)
            VALUES
            (?1, ?2)",
        (&task.t_name, &task.t_priority),
    )
}

impl Default for Task {
    fn default() -> Self {
        Task {
            t_id: 1,
            t_name: "".to_string(),
            t_priority: 0,
        }
    }
}
