use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Session {
    pub _s_id: i64,
    pub s_task: i64,
    pub s_user: i64,
    pub s_duration: u64,
    pub s_comment: String,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            _s_id: 1,
            s_task: 1,
            s_user: 1,
            s_duration: 0,
            s_comment: "".to_string(),
        }
    }
}

pub fn save_session(conn: &Connection, session: &Session) -> Result<usize> {
    conn.execute(
        "INSERT INTO sessions
            (s_task, s_user, s_duration, s_comment)
            VALUES
            (?1, ?2, ?3, ?4)",
        (
            session.s_task,
            session.s_user,
            session.s_duration,
            &session.s_comment,
        ),
    )
}
