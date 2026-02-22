use std::sync::{Arc, Mutex};

use tauri::{State, Window};

use crate::{
    conversation::{models::Conversation, service},
    infrastructure::context::Context,
};

#[tauri::command]
pub async fn start_conversation(
    title: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<String, String> {
    let ctx = &mut app_state.lock().unwrap();
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

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let mut ctx = app_state_handle.lock().unwrap();
        let result = service::continue_conversation(&conv_id, &user_input, window, &mut ctx);
        let _ = tx.send(result);
    });

    rx.recv()
        .map_err(|e| format!("Download failed: {}", e))?
        .map_err(|e| format!("Model fetch failed: {:?}", e))?;

    Ok(None)
}

#[tauri::command]
pub fn get_conversation_ids(app_state: State<'_, Arc<Mutex<Context>>>) -> Vec<String> {
    let ctx = &mut *app_state.lock().unwrap();
    service::get_conversation_ids(ctx)
}

#[tauri::command]
pub fn get_conversation(
    conv_id: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<Option<Conversation>, String> {
    let ctx = &mut *app_state.lock().unwrap();
    service::get_conversation(&conv_id, ctx).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_conversation(
    conv_id: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<String, String> {
    let ctx = &mut *app_state.lock().unwrap();
    service::delete_conversation(&conv_id, ctx).map_err(|e| e.to_string())
}
