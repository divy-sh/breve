use std::sync::Arc;

use tauri::{State, async_runtime::Mutex};

use crate::{infrastructure::context::Context, settings::service};

#[tauri::command]
pub async fn get_config(key: String, app_state: State<'_, Arc<Mutex<Context>>>) -> Result<String, String> {
    let ctx = &mut app_state.lock().await;
    service::get_config(key, ctx).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_config(
    key: String,
    value: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<(), String> {
    let ctx = &mut app_state.lock().await;
    service::set_config(key, value, ctx).map_err(|e| e.to_string())
}
