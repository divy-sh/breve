use std::sync::Mutex;

use tauri::{State, Window};

use crate::{
    conversation::{models::Conversation, service},
    infrastructure::app::App,
};

#[tauri::command]
pub async fn start_conversation(
    title: String,
    app_state: State<'_, Mutex<App>>,
) -> Result<String, String> {
    let app = &mut app_state.lock().unwrap();
    service::start_new_conversation(&title, app).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn continue_conversation(
    conv_id: String,
    user_input: String,
    window: Window,
    app_state: State<'_, Mutex<App>>,
) -> Result<Option<String>, String> {
    let app = &mut app_state.lock().unwrap();
    service::continue_conversation(&conv_id, &user_input, window, app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_conversation_ids(app_state: State<'_, Mutex<App>>) -> Vec<String> {
    let app = &mut *app_state.lock().unwrap();
    service::get_conversation_ids(app)
}

#[tauri::command]
pub fn get_conversation(
    conv_id: String,
    app_state: State<'_, Mutex<App>>,
) -> Result<Option<Conversation>, String> {
    let app = &mut *app_state.lock().unwrap();
    service::get_conversation(&conv_id, app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_conversation(
    conv_id: String,
    app_state: State<'_, Mutex<App>>,
) -> Result<String, String> {
    let app = &mut *app_state.lock().unwrap();
    service::delete_conversation(&conv_id, app).map_err(|e| e.to_string())
}
