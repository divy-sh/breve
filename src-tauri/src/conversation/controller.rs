use std::sync::Arc;

use tauri::{State, Window, async_runtime::Mutex};

use crate::{
    conversation::{models::Conversation, service},
    infrastructure::context::Context,
};

#[tauri::command]
pub async fn start_conversation(
    title: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<String, String> {
    let ctx = &mut app_state.lock().await;
    service::start_new_conversation(&title, ctx).map_err(|e| e.to_string())
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
    }).await.map_err(|e| e.to_string())?;

    result.map_err(|e| format!("Inference Failed: {:?}", e))?;
    Ok(None)
}

#[tauri::command]
pub async fn get_conversation_ids(app_state: State<'_, Arc<Mutex<Context>>>) -> Result<Vec<String>, String> {
    let ctx = &mut *app_state.lock().await;
    Ok(service::get_conversation_ids(ctx))
}

#[tauri::command]
pub async fn get_conversation(
    conv_id: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<Option<Conversation>, String> {
    let ctx = &mut *app_state.lock().await;
    service::get_conversation(&conv_id, ctx).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_conversation(
    conv_id: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<String, String> {
    let ctx = &mut *app_state.lock().await;
    service::delete_conversation(&conv_id, ctx).map_err(|e| e.to_string())
}
