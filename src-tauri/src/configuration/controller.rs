use std::{collections::HashMap, sync::Arc};

use tauri::{State, async_runtime::Mutex};

use crate::{
    configuration::{models::Config, repository},
    inference,
    infrastructure::context::Context,
};
use serde_json::Value;

#[tauri::command]
pub async fn get_model_config(app_state: State<'_, Arc<Mutex<Context>>>) -> Result<Config, String> {
    let ctx = app_state.lock().await;

    return Ok(ctx.config.clone());
}

#[tauri::command]
pub async fn set_model_config(
    app_state: State<'_, Arc<Mutex<Context>>>,
    payload: HashMap<String, Value>,
) -> Result<(), String> {
    let mut ctx = app_state.lock().await;

    let mut current_config_json = serde_json::to_value(&ctx.config).map_err(|e| e.to_string())?;

    if let Some(obj) = current_config_json.as_object_mut() {
        for (key, value) in payload.clone() {
            obj.insert(key, value);
        }
    }

    let updated_config: Config = serde_json::from_value(current_config_json)
        .map_err(|e| format!("Validation failed: {}", e))?;

    for (key, val) in payload {
        let _ = repository::set_model_config(key, val.to_string());
    }
    ctx.config = updated_config;
    inference::service::activate_model(ctx.config.default_model.clone(), &mut ctx)
}
