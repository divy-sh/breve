use std::sync::{Arc, Mutex};

use tauri::State;

use crate::{infrastructure::context::Context, settings::service};

#[tauri::command]
pub fn get_config(key: String, app_state: State<'_, Arc<Mutex<Context>>>) -> Result<String, String> {
    let ctx = &mut app_state.lock().unwrap();
    service::get_config(key, ctx).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_config(
    key: String,
    value: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<(), String> {
    let ctx = &mut app_state.lock().unwrap();
    service::set_config(key, value, ctx).map_err(|e| e.to_string())
}
