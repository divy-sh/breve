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
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<String, String> {
    // clone the Arc to get ownership for locking
    let state_arc = Arc::clone(&state.inner());
    match state_arc.lock() {
        Ok(mut controller) => controller
            .start_new_conversation(&title)
            .map_err(|e| e.to_string()),
        Err(_) => Err("Internal error: controller lock poisoned".to_string()),
    }
}

#[tauri::command]
async fn continue_conversation(
    conv_id: String,
    user_input: String,
    window: Window,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<Option<String>, String> {
    let state_arc = Arc::clone(&state.inner());
    match state_arc.lock() {
        Ok(mut controller) => controller
            .continue_conversation(&conv_id, &user_input, window)
            .map_err(|e| e.to_string()),
        Err(_) => Err("Internal error: controller lock poisoned".to_string()),
    }
}

#[tauri::command]
fn get_conversation_ids(state: State<'_, Arc<Mutex<ConversationController>>>) -> Vec<String> {
    let state_arc = Arc::clone(&state.inner());
    match state_arc.lock() {
        Ok(controller) => controller.get_conversation_ids(),
        Err(_) => vec![],
    }
}

/// Ensure model is available. This spawns a background thread to download the model
/// if it's missing and emits `downloading-model` events to the window. Returns immediately.
#[tauri::command]
fn ensure_model(window: Window, state: tauri::State<Arc<Mutex<ConversationController>>>) -> Result<(), String> {
    let state_arc = Arc::clone(&state.inner());
    let window_for_thread = window.clone();
    std::thread::spawn(move || {
        // Acquire the lock only briefly to read config and check existing inference.
        let config = match state_arc.lock() {
            Ok(controller) => {
                // If inference is already initialized, nothing to do.
                if controller.inference.is_some() {
                    return;
                }
                controller.config.clone()
            }
            Err(_) => {
                let _ = window_for_thread.emit("downloading-model", false);
                eprintln!("Failed to acquire controller lock in ensure_model");
                return;
            }
        };

        // fetch_model will emit events to the provided window as it runs. Do this without holding the controller lock.
        if let Err(e) = models::model_fetcher::ModelFetcher::fetch_model(
            &config.model_url,
            &config.model_name,
            &config.get_model_path(),
            window_for_thread.clone(),
        ) {
            eprintln!("Model fetch failed: {:?}", e);
            // In case of error, make sure to notify frontend to stop loading state
            let _ = window_for_thread.emit("downloading-model", false);
            return;
        }

        // Initialize inference (may be expensive) without holding the controller lock.
        match Inference::init(&config) {
            Ok(inference) => {
                // Reacquire the lock only to set the inference instance.
                match state_arc.lock() {
                    Ok(mut controller) => {
                        controller.set_inference(inference);
                    }
                    Err(_) => {
                        eprintln!("Failed to acquire controller lock to set inference");
                        let _ = window_for_thread.emit("downloading-model", false);
                    }
                }
            }
            Err(e) => {
                eprintln!("Inference init failed: {}", e);
                let _ = window_for_thread.emit("downloading-model", false);
            }
        }
    });
    Ok(())
}

#[tauri::command]
fn get_conversation(
    conv_id: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<Option<models::conversation::Conversation>, String> {
    let state_arc = Arc::clone(&state.inner());
    match state_arc.lock() {
        Ok(controller) => controller
            .get_conversation(&conv_id)
            .map_err(|e| e.to_string()),
        Err(_) => Err("Internal error: controller lock poisoned".to_string()),
    }
}

#[tauri::command]
fn delete_conversation(
    conv_id: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<String, String> {
    let state_arc = Arc::clone(&state.inner());
    match state_arc.lock() {
        Ok(controller) => controller
            .delete_conversation(&conv_id)
            .map_err(|e| e.to_string()),
        Err(_) => Err("Internal error: controller lock poisoned".to_string()),
    }
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
