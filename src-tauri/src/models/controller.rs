use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tauri::{State, Window};

use crate::{
    inference,
    infrastructure::{context::Context, path_resolver},
    models::{
        self, models::Model, service::{SET, UNSET}
    },
    settings,
};

#[tauri::command]
pub fn get_available_models(
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> &'static HashMap<String, Model> {
    let ctx = &mut app_state.lock().unwrap();
    ctx.config.get_available_models()
}

#[tauri::command]
pub fn get_default_model(app_state: State<'_, Arc<Mutex<Context>>>) -> String {
    let ctx = &mut app_state.lock().unwrap();
    ctx.config.default_model.clone()
}

#[tauri::command]
pub fn get_model_status(app_state: State<'_, Arc<Mutex<Context>>>) -> String {
    let ctx = &mut app_state.lock().unwrap();
    let name = &ctx.config.default_model;
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
pub fn list_downloaded_models(app_state: State<'_, Arc<Mutex<Context>>>) -> Vec<String> {
    let ctx = &mut app_state.lock().unwrap();
    let cfg = &ctx.config;
    let mut found = vec![];
    for (name, _) in cfg.get_available_models() {
        let p = path_resolver::paths().app_local_data(&name).unwrap();
        if p.exists() {
            found.push(name.to_string());
        }
    }
    found
}

#[tauri::command]
pub async fn download_model(
    model_name: String,
    window: Window,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<(), String> {
    let url;
    let path;
    {
        let ctx = &mut app_state.lock().unwrap();
        let cfg = &ctx.config;
        url = cfg
        .get_available_models()
        .get(&model_name)
        .ok_or("Model not found")?
        .repo
        .clone();
        path = path_resolver::paths()
        .app_local_data(&model_name)
        .unwrap()
        .to_string_lossy()
        .to_string();
    }

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
pub fn delete_model(model_name: String, app_state: State<'_, Arc<Mutex<Context>>>) -> Result<(), String> {
    let ctx = &mut app_state.lock().unwrap();
    let config = &mut ctx.config;
    let path = path_resolver::paths().app_local_data(&model_name).unwrap();

    if path.exists() {
        std::fs::remove_file(&path)
            .or_else(|_| std::fs::remove_dir_all(&path))
            .map_err(|e| format!("Delete failed: {}", e))?;
    }

    if config.default_model == model_name {
        config.default_model.clear();
        ctx.inference = None;

        settings::service::set_config("model_name".into(), "".into(), ctx)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn set_default_model(
    model_name: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<(), String> {

    let ctx_clone = Arc::clone(&app_state);
    std::thread::spawn(move || {
        let mut ctx = ctx_clone.lock().unwrap();
        inference::service::activate_model(model_name, &mut ctx)
    });
    Ok(())
}
