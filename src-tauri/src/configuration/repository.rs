use rusqlite::Result;

use crate::infrastructure::database::Database;

pub fn get_model_config(name: String) -> Result<Option<String>> {
    let conn = Database::get_db().get_conn();
    let mut stmt = conn.prepare("SELECT value FROM model_config WHERE key = ?1")?;
    let mut rows = stmt.query(rusqlite::params![name])?;

    if let Some(row) = rows.next()? {
        let value: String = row.get(0)?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub fn set_model_config(name: String, value: String) -> Result<()> {
    let conn = Database::get_db().get_conn();
    conn.execute(
        "INSERT INTO model_config (key, value) VALUES (?1, ?2)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![name, value],
    )?;
    Ok(())
}
