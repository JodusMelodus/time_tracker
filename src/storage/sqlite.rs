use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("data/sessions.db")?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            task_id TEXT,
            start_utc TEXT NOT NULL,
            end_utc TEXT,
            duration_sec INTEGER,
            synced INTEGER DEFAULT 0
        );
        PRAGMA journal_mode=WAL;
    ",
    )?;
    Ok(conn)
}
