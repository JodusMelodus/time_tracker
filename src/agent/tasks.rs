use rusqlite::{Connection, Result};

pub struct Task {
    pub _id: isize,
    pub name: String,
    pub description: String,
}

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut statement = conn.prepare("SELECT * FROM tasks")?;
    let task_iter = statement.query_map([], |row| {
        Ok(Task {
            _id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
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
        "INSERT INTO tasks (name, description) VALUES (?1, ?2)",
        (&task.name, &task.description),
    )
}

impl Default for Task {
    fn default() -> Self {
        Task {
            _id: 0,
            name: "".to_string(),
            description: "".to_string(),
        }
    }
}
