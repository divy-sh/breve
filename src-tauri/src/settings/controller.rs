use std::sync::Mutex;

use tauri::State;

use crate::{infrastructure::app::App, settings::service};

#[tauri::command]
pub fn get_config(key: String, app_state: State<'_, Mutex<App>>) -> Result<String, String> {
    let app = &mut app_state.lock().unwrap();
    service::get_config(key, app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_config(
    key: String,
    value: String,
    app_state: State<'_, Mutex<App>>,
) -> Result<(), String> {
    let app = &mut app_state.lock().unwrap();
    service::set_config(key, value, app).map_err(|e| e.to_string())
}
