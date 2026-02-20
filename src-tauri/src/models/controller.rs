use std::{
    collections::{BTreeMap, HashMap},
    sync::Mutex,
};

use tauri::{State, Window};

use crate::{
    inference,
    infrastructure::{app::App, path_resolver},
    models::{
        self,
        service::{SET, UNSET},
    },
    settings,
};

#[tauri::command]
pub fn get_available_models(
    app_state: State<'_, Mutex<App>>,
) -> BTreeMap<String, HashMap<String, String>> {
    let app = &mut app_state.lock().unwrap();
    app.config.get_available_models()
}

#[tauri::command]
pub fn get_default_model(app_state: State<'_, Mutex<App>>) -> String {
    let app = &mut app_state.lock().unwrap();
    app.config.model_name.clone()
}

#[tauri::command]
pub fn get_model_status(app_state: State<'_, Mutex<App>>) -> String {
    let app = &mut app_state.lock().unwrap();
    let name = &app.config.model_name;
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
pub fn list_downloaded_models(app_state: State<'_, Mutex<App>>) -> Vec<String> {
    let app = &mut app_state.lock().unwrap();
    let cfg = &app.config;
    let mut found = vec![];
    for (name, _) in cfg.get_available_models() {
        let p = path_resolver::paths().app_local_data(&name).unwrap();
        if p.exists() {
            found.push(name);
        }
    }
    found
}

#[tauri::command]
pub async fn download_model(
    model_name: String,
    window: Window,
    app_state: State<'_, Mutex<App>>,
) -> Result<(), String> {
    let app = &mut app_state.lock().unwrap();
    let cfg = &app.config;
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
pub fn delete_model(model_name: String, app_state: State<'_, Mutex<App>>) -> Result<(), String> {
    let app = &mut app_state.lock().unwrap();
    let config = &mut app.config;
    let path = path_resolver::paths().app_local_data(&model_name).unwrap();

    if path.exists() {
        std::fs::remove_file(&path)
            .or_else(|_| std::fs::remove_dir_all(&path))
            .map_err(|e| format!("Delete failed: {}", e))?;
    }

    if config.model_name == model_name {
        config.model_name.clear();
        app.inference = None;

        settings::service::set_config("model_name".into(), "".into(), app)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn set_default_model(
    model_name: String,
    app_state: State<'_, Mutex<App>>,
) -> Result<(), String> {
    let app = &mut app_state.lock().unwrap();
    inference::service::activate_model(model_name, app)
}
