use std::sync::Mutex;

use config::path_resolver::init_app_paths;
use controllers::conversation_controller::ConversationController;
use tauri::{Manager, State, Window};

pub mod config;
pub mod controllers;
pub mod dao;
pub mod models;

#[tauri::command]
async fn start_conversation(
    title: String,
    state: State<'_, Mutex<ConversationController>>,
) -> Result<String, String> {
    let mut controller = state.lock().unwrap();
    controller
        .start_new_conversation(&title)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn continue_conversation(
    conv_id: String,
    user_input: String,
    window: Window,
    state: State<'_, Mutex<ConversationController>>,
) -> Result<Option<String>, String> {
    let mut controller = state.lock().unwrap();
    controller
        .continue_conversation(&conv_id, &user_input, window)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_conversation_ids(state: State<'_, Mutex<ConversationController>>) -> Vec<String> {
    let controller = state.lock().unwrap();
    controller.get_conversation_ids()
}

#[tauri::command]
fn get_conversation(
    conv_id: String,
    state: State<'_, Mutex<ConversationController>>,
) -> Result<Option<models::conversation::Conversation>, String> {
    let controller = state.lock().unwrap();
    controller
        .get_conversation(&conv_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_conversation(
    conv_id: String,
    state: State<'_, Mutex<ConversationController>>,
) -> Result<String, String> {
    let controller = state.lock().unwrap();
    controller
        .delete_conversation(&conv_id)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            init_app_paths(app.handle().clone());
            app.manage(Mutex::new(ConversationController::new()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_conversation,
            continue_conversation,
            get_conversation_ids,
            get_conversation,
            delete_conversation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
