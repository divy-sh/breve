use std::sync::Arc;

use tauri::{State, Window, async_runtime::Mutex};

use crate::{
    conversation::{models::Conversation, service},
    infrastructure::context::Context,
};

#[tauri::command]
pub async fn start_conversation(title: String) -> Result<String, String> {
    service::start_new_conversation(&title).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn continue_conversation(
    conv_id: String,
    user_input: String,
    window: Window,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<Option<String>, String> {
    let app_state_handle: Arc<Mutex<Context>> = app_state.inner().clone();

    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut ctx = tauri::async_runtime::block_on(app_state_handle.lock());
        service::continue_conversation(&conv_id, &user_input, window, &mut ctx)
    })
    .await
    .map_err(|e| e.to_string())?;

    result.map_err(|e| format!("Inference Failed: {:?}", e))?;
    Ok(None)
}

#[tauri::command]
pub async fn get_conversation_ids() -> Result<Vec<String>, String> {
    Ok(service::get_conversation_ids())
}

#[tauri::command]
pub async fn get_conversation(conv_id: String) -> Result<Option<Conversation>, String> {
    service::get_conversation(&conv_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_conversation(conv_id: String) -> Result<String, String> {
    service::delete_conversation(&conv_id).map_err(|e| e.to_string())
}
