use rusqlite::Result;

use crate::infrastructure::repository as dao;

pub fn get_config(key: String) -> Result<String, String> {
    dao::get_config(key.clone())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Config not found for key {}", key).to_string())
}

pub fn set_config(key: String, value: String) -> Result<(), String> {
    dao::set_config(key, value).map_err(|e| e.to_string())
}
