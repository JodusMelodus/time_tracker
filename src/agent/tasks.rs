use rusqlite::{Connection, Result};

pub struct Task {
    pub id: String,
    pub name: String,
    pub description: String,
}

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut statement = conn.prepare("SELECT * FROM tasks")?;
    let task_iter = statement.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
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
