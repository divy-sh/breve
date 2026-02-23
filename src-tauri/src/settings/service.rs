use rusqlite::Result;

use crate::settings::repository as dao;

pub fn get_config(key: String) -> Result<String, String> {
    println!("Getting config for key: {}", key);
    dao::get_config(key)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Config not found".to_string())
}

pub fn set_config(key: String, value: String) -> Result<(), String> {
    dao::set_config(key, value).map_err(|e| e.to_string())
}
