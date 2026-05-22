use std::collections::HashMap;

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

pub fn save_user_model(name: String, path: String) -> Result<(), String> {
    dao::save_user_model(name, path).map_err(|e| e.to_string())
}

pub fn get_user_models() -> Result<HashMap<String, String>> {
    dao::get_user_models()
}
