use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("Failed to connect to database");
        _ = init_conversation_dao(&conn);
        _ = init_settings_dao(&conn);

        Database { conn }
    }
}

pub fn init_conversation_dao(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            lastUpdated TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn init_settings_dao(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}
