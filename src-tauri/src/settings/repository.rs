use rusqlite::{Connection, Result};

pub fn get_config(name: String, conn: &Connection) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE name = ?1")?;
    let mut rows = stmt.query(rusqlite::params![name])?;

    if let Some(row) = rows.next()? {
        let value: String = row.get(0)?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub fn set_config(name: String, value: String, conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (name, value) VALUES (?1, ?2)
            ON CONFLICT(name) DO UPDATE SET value = excluded.value",
        rusqlite::params![name, value],
    )?;
    Ok(())
}
