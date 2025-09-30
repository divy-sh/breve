use std::sync::{Arc, Mutex};

use config::path_resolver::init_app_paths;
use controllers::conversation_controller::ConversationController;
use tauri::{Manager, State, Window, Emitter};

use crate::models::inference::Inference;

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

/// Ensure model is available. This spawns a background thread to download the model
/// if it's missing and emits `downloading-model` events to the window. Returns immediately.
#[tauri::command]
fn ensure_model(window: Window, state: tauri::State<Arc<Mutex<ConversationController>>>) -> Result<(), String> {
    let state_arc = Arc::clone(&state.inner());
    let window_for_thread = window.clone();
    std::thread::spawn(move || {
        let mut controller = state_arc.lock().unwrap();
        let config = controller.config.clone();
        // fetch_model will emit events to the provided window as it runs
        if let Err(e) = models::model_fetcher::ModelFetcher::fetch_model(
            &config.model_url,
            &config.model_name,
            &config.get_model_path(),
            window_for_thread.clone(),
        ) {
            eprintln!("Model fetch failed: {:?}", e);
            // In case of error, make sure to notify frontend to stop loading state
            let _ = window_for_thread.emit("downloading-model", false);
        } else {
            // Avoid borrowing controller.config while mutably borrowing controller
            if let Ok(inference) = Inference::init(&config) {
                controller.set_inference(inference);
            }
        }
    });
    Ok(())
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
    use std::sync::Arc;
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            init_app_paths(app.handle().clone());
            app.manage(Arc::new(Mutex::new(ConversationController::new())));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_conversation,
            continue_conversation,
            get_conversation_ids,
            ensure_model,
            get_conversation,
            delete_conversation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
