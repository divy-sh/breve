use std::sync::Arc;

use tauri::{State, async_runtime::Mutex};

use crate::{configuration::models::Config, infrastructure::context::Context};

#[tauri::command]
pub async fn get_model_config(app_state: State<'_, Arc<Mutex<Context>>>) -> Result<Config, String> {
    let ctx = app_state.lock().await;

    return Ok(ctx.config.clone());
}
