use std::sync::{Arc, Mutex};

use tauri::{State, Window};

use crate::conversation::{models::Conversation, service::ConversationController};

#[tauri::command]
pub async fn start_conversation(
    title: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<String, String> {
    let state_arc = Arc::clone(state.inner());
    let mut controller = state_arc.lock().map_err(|_| "Lock poisoned")?;
    controller
        .start_new_conversation(&title)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn continue_conversation(
    conv_id: String,
    user_input: String,
    window: Window,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<Option<String>, String> {
    let state_arc = Arc::clone(state.inner());
    let mut controller = state_arc.lock().map_err(|_| "Lock poisoned")?;
    controller
        .continue_conversation(&conv_id, &user_input, window)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_conversation_ids(state: State<'_, Arc<Mutex<ConversationController>>>) -> Vec<String> {
    state
        .lock()
        .map(|c| c.get_conversation_ids())
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_conversation(
    conv_id: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<Option<Conversation>, String> {
    let controller = state.lock().map_err(|_| "Lock poisoned")?;
    controller
        .get_conversation(&conv_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_conversation(
    conv_id: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<String, String> {
    let controller = state.lock().map_err(|_| "Lock poisoned")?;
    controller
        .delete_conversation(&conv_id)
        .map_err(|e| e.to_string())
}
