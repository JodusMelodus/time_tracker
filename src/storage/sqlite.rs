use rusqlite::{Connection, Result};
use std::fs;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("data/sessions.db")?;

    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        ",
    )?;

    let schema = fs::read_to_string("schema.sql").unwrap();
    conn.execute_batch(&schema)?;
    Ok(conn)
}
