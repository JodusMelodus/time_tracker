use rusqlite::{Connection, Result};
use std::sync::Arc;

use crate::{DB_SCHEMA, config};

pub fn init_db(settings: Arc<config::settings::Settings>) -> Result<Connection> {
    let conn = Connection::open(&settings.local_database_path)?;

    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        ",
    )?;

    conn.execute_batch(DB_SCHEMA)?;
    // Ensure the current user exists so sessions can reference it (foreign key)
    conn.execute(
        "INSERT OR IGNORE INTO users (u_id, u_name) VALUES (?1, ?2)",
        (&settings.uid, &settings.uid),
    )?;
    Ok(conn)
}
