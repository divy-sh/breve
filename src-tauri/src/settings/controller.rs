use std::sync::{Arc, Mutex};

use tauri::State;

use crate::settings::service::SettingsController;

#[tauri::command]
pub fn get_config(
    key: String,
    state: State<'_, Arc<Mutex<SettingsController>>>,
) -> Result<String, String> {
    let controller = state.lock().map_err(|_| "Lock poisoned")?;
    controller.get_config(key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_config(
    key: String,
    value: String,
    state: State<'_, Arc<Mutex<SettingsController>>>,
) -> Result<(), String> {
    let controller = state.lock().map_err(|_| "Lock poisoned")?;
    controller.set_config(key, value).map_err(|e| e.to_string())
}
