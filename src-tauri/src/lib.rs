use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
};

use config::config_handler::Config;
use config::path_resolver::init_app_paths;
use controllers::conversation_controller::ConversationController;
use tauri::{Manager, State, Window};

use crate::{
    controllers::settings_controller::SettingsController,
    models::{
        inference::Inference,
        model_fetcher::{SET, UNSET},
    },
};

pub mod config;
pub mod controllers;
pub mod dao;
pub mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_iap::init())
        .setup(|app| {
            init_app_paths(app.handle().clone());

            app.manage(Arc::new(Mutex::new(ConversationController::new())));
            app.manage(Arc::new(Mutex::new(SettingsController::new())));

            let settings_state = app.state::<Arc<Mutex<SettingsController>>>();
            let convo_state = app.state::<Arc<Mutex<ConversationController>>>();

            let saved_model = {
                let settings_lock = settings_state.lock().ok();
                settings_lock.and_then(|s| s.get_config("model_name".to_string()).ok())
            };

            if let Some(model_name) = saved_model {
                let _ = activate_model(
                    model_name,
                    Arc::clone(settings_state.inner()),
                    Arc::clone(convo_state.inner()),
                );
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_conversation,
            continue_conversation,
            get_conversation_ids,
            get_conversation,
            delete_conversation,
            get_model_status,
            get_config,
            set_config,
            get_available_models,
            list_downloaded_models,
            download_model,
            delete_model,
            set_default_model,
            get_default_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn start_conversation(
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
async fn continue_conversation(
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
fn get_conversation_ids(state: State<'_, Arc<Mutex<ConversationController>>>) -> Vec<String> {
    state
        .lock()
        .map(|c| c.get_conversation_ids())
        .unwrap_or_default()
}

#[tauri::command]
fn get_conversation(
    conv_id: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<Option<models::conversation::Conversation>, String> {
    let controller = state.lock().map_err(|_| "Lock poisoned")?;
    controller
        .get_conversation(&conv_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_conversation(
    conv_id: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<String, String> {
    let controller = state.lock().map_err(|_| "Lock poisoned")?;
    controller
        .delete_conversation(&conv_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_config(
    key: String,
    state: State<'_, Arc<Mutex<SettingsController>>>,
) -> Result<String, String> {
    let controller = state.lock().map_err(|_| "Lock poisoned")?;
    controller.get_config(key).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_config(
    key: String,
    value: String,
    state: State<'_, Arc<Mutex<SettingsController>>>,
) -> Result<(), String> {
    let controller = state.lock().map_err(|_| "Lock poisoned")?;
    controller.set_config(key, value).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_available_models() -> BTreeMap<String, HashMap<String, String>> {
    Config::init()
        .map(|c| c.get_available_models())
        .unwrap_or_default()
}

#[tauri::command]
fn get_default_model(state: State<'_, Arc<Mutex<ConversationController>>>) -> String {
    state
        .lock()
        .map(|c| c.config.model_name.clone())
        .unwrap_or_default()
}

#[tauri::command]
fn get_model_status(state: State<'_, Arc<Mutex<ConversationController>>>) -> String {
    let controller = match state.lock() {
        Ok(c) => c,
        Err(_) => return UNSET.into(),
    };

    let name = controller.config.model_name.clone();
    if name.is_empty() {
        return UNSET.into();
    }

    let path = crate::config::path_resolver::paths()
        .app_local_data(&name)
        .unwrap();
    if path.exists() {
        SET.into()
    } else {
        UNSET.into()
    }
}

#[tauri::command]
fn list_downloaded_models() -> Vec<String> {
    let mut found = vec![];

    if let Ok(cfg) = Config::init() {
        for (name, _) in cfg.get_available_models() {
            let p = crate::config::path_resolver::paths()
                .app_local_data(&name)
                .unwrap();
            if p.exists() {
                found.push(name);
            }
        }
    }
    found
}

#[tauri::command]
async fn download_model(model_name: String, window: Window) -> Result<(), String> {
    let cfg = Config::init().map_err(|e| e.to_string())?;
    let url = cfg
        .get_available_models()
        .get(&model_name)
        .ok_or("Model not found")?
        .get("repo")
        .ok_or("Repo URL not found")?
        .clone();
    let path = crate::config::path_resolver::paths()
        .app_local_data(&model_name)
        .unwrap()
        .to_string_lossy()
        .to_string();

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let result =
            models::model_fetcher::ModelFetcher::fetch_model(&url, &model_name, &path, window);
        let _ = tx.send(result);
    });

    rx.recv()
        .map_err(|e| format!("Download failed: {}", e))?
        .map_err(|e| format!("Model fetch failed: {:?}", e))?;

    Ok(())
}

#[tauri::command]
fn delete_model(
    model_name: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
    settings: State<'_, Arc<Mutex<SettingsController>>>,
) -> Result<(), String> {
    let path = crate::config::path_resolver::paths()
        .app_local_data(&model_name)
        .unwrap();

    if path.exists() {
        std::fs::remove_file(&path)
            .or_else(|_| std::fs::remove_dir_all(&path))
            .map_err(|e| format!("Delete failed: {}", e))?;
    }

    let mut controller = state.lock().map_err(|_| "Lock poisoned")?;
    if controller.config.model_name == model_name {
        controller.config.model_name.clear();
        controller.inference = None;

        if let Ok(settings) = settings.lock() {
            let _ = settings.set_config("model_name".into(), "".into());
        }
    }

    Ok(())
}

#[tauri::command]
fn set_default_model(
    model_name: String,
    settings: State<'_, Arc<Mutex<SettingsController>>>,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<(), String> {
    activate_model(
        model_name,
        Arc::clone(settings.inner()),
        Arc::clone(state.inner()),
    )
}

fn validate_model(config: &Config, model_name: &str) -> Result<(), String> {
    if !config.get_available_models().contains_key(model_name) {
        return Err("Model not available".into());
    }

    let path = crate::config::path_resolver::paths()
        .app_local_data(model_name)
        .unwrap();

    if !path.exists() {
        return Err("Model not downloaded".into());
    }

    Ok(())
}

fn activate_model(
    model_name: String,
    settings: Arc<Mutex<SettingsController>>,
    state: Arc<Mutex<ConversationController>>,
) -> Result<(), String> {
    let cfg = {
        let controller = state.lock().map_err(|_| "Controller lock poisoned")?;
        controller.config.clone()
    };

    validate_model(&cfg, &model_name)?;
    // persist
    settings
        .lock()
        .map_err(|_| "Settings lock poisoned")?
        .set_config("model_name".into(), model_name.clone())
        .map_err(|e| e.to_string())?;

    // update controller
    {
        let mut controller = state.lock().map_err(|_| "Controller lock poisoned")?;

        if controller.config.model_name == model_name && controller.inference.is_some() {
            return Ok(());
        }

        controller.set_model_name(model_name.clone());
        controller.inference = None;
    }

    // init inference async
    let state_clone = Arc::clone(&state);
    std::thread::spawn(move || {
        let cfg = match state_clone.lock() {
            Ok(c) => c.config.clone(),
            Err(_) => return,
        };

        initialize_inference(cfg, Arc::clone(&state_clone));
    });

    Ok(())
}

fn initialize_inference(config: Config, state_arc: Arc<Mutex<ConversationController>>) {
    match Inference::init(&config) {
        Ok(inference) => {
            if let Ok(mut controller) = state_arc.lock() {
                controller.set_inference(inference);
            }
        }
        Err(e) => eprintln!("Inference init failed: {}", e),
    }
}
