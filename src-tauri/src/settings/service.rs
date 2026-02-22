use rusqlite::Result;

use crate::{infrastructure::context::Context, settings::repository as dao};

pub fn get_config(key: String, app: &mut Context) -> Result<String, String> {
    println!("Getting config for key: {}", key);
    dao::get_config(key, &app.db.conn)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Config not found".to_string())
}

pub fn set_config(key: String, value: String, app: &mut Context) -> Result<(), String> {
    dao::set_config(key, value, &app.db.conn).map_err(|e| e.to_string())
}
