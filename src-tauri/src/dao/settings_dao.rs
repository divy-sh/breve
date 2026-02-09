use crate::config::config_handler::Config;
use rusqlite::{Connection, Result};

pub struct SettingsDao {
    conn: Connection,
}

impl SettingsDao {
    pub fn init(config: &Config) -> Result<Self> {
        let conn = Connection::open(config.get_db_path())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                name TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        Ok(SettingsDao { conn })
    }

    pub fn get_config(&self, name: String) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM settings WHERE name = ?1")?;
        let mut rows = stmt.query(rusqlite::params![name])?;

        if let Some(row) = rows.next()? {
            let value: String = row.get(0)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub fn set_config(&self, name: String, value: String) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings (name, value) VALUES (?1, ?2)
             ON CONFLICT(name) DO UPDATE SET value = excluded.value",
            rusqlite::params![name, value],
        )?;
        Ok(())
    }
}
