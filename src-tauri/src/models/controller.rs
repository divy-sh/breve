use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
};

use tauri::{State, Window};

use crate::{
    conversation::service::ConversationController,
    inference,
    infrastructure::{config::Config, path_resolver},
    models::{
        self, service::{SET, UNSET}
    },
    settings::service::SettingsController,
};

#[tauri::command]
pub fn get_available_models() -> BTreeMap<String, HashMap<String, String>> {
    Config::init()
        .map(|c| c.get_available_models())
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_default_model(state: State<'_, Arc<Mutex<ConversationController>>>) -> String {
    state
        .lock()
        .map(|c| c.config.model_name.clone())
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_model_status(state: State<'_, Arc<Mutex<ConversationController>>>) -> String {
    let controller = match state.lock() {
        Ok(c) => c,
        Err(_) => return UNSET.into(),
    };

    let name = controller.config.model_name.clone();
    if name.is_empty() {
        return UNSET.into();
    }

    let path = path_resolver::paths().app_local_data(&name).unwrap();
    if path.exists() {
        SET.into()
    } else {
        UNSET.into()
    }
}

#[tauri::command]
pub fn list_downloaded_models() -> Vec<String> {
    let mut found = vec![];

    if let Ok(cfg) = Config::init() {
        for (name, _) in cfg.get_available_models() {
            let p = path_resolver::paths().app_local_data(&name).unwrap();
            if p.exists() {
                found.push(name);
            }
        }
    }
    found
}

#[tauri::command]
pub async fn download_model(model_name: String, window: Window) -> Result<(), String> {
    let cfg = Config::init().map_err(|e| e.to_string())?;
    let url = cfg
        .get_available_models()
        .get(&model_name)
        .ok_or("Model not found")?
        .get("repo")
        .ok_or("Repo URL not found")?
        .clone();
    let path = path_resolver::paths()
        .app_local_data(&model_name)
        .unwrap()
        .to_string_lossy()
        .to_string();

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let result = models::service::fetch_model(&url, &model_name, &path, window);
        let _ = tx.send(result);
    });

    rx.recv()
        .map_err(|e| format!("Download failed: {}", e))?
        .map_err(|e| format!("Model fetch failed: {:?}", e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_model(
    model_name: String,
    state: State<'_, Arc<Mutex<ConversationController>>>,
    settings: State<'_, Arc<Mutex<SettingsController>>>,
) -> Result<(), String> {
    let path = path_resolver::paths().app_local_data(&model_name).unwrap();

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
pub fn set_default_model(
    model_name: String,
    settings: State<'_, Arc<Mutex<SettingsController>>>,
    state: State<'_, Arc<Mutex<ConversationController>>>,
) -> Result<(), String> {
    inference::service::activate_model(
        model_name,
        Arc::clone(settings.inner()),
        Arc::clone(state.inner()),
    )
}
