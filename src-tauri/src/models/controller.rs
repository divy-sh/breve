use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use tauri::{State, Window, async_runtime::Mutex};

use crate::{
    inference,
    infrastructure::{self, context::Context, path_resolver},
    models::{
        self,
        models::Model,
        service::{SET, UNSET},
    },
};

#[tauri::command]
pub async fn get_available_models(
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<&'static HashMap<String, Model>, String> {
    let ctx = &mut app_state.lock().await;
    Ok(ctx.config.get_available_models())
}

#[tauri::command]
pub async fn get_default_model(
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<String, String> {
    let ctx = &mut app_state.lock().await;
    Ok(ctx.config.default_model.clone())
}

#[tauri::command]
pub async fn get_model_status(app_state: State<'_, Arc<Mutex<Context>>>) -> Result<String, String> {
    let ctx = &mut app_state.lock().await;
    let name = &ctx.config.default_model;
    if name.is_empty() {
        return Ok(UNSET.into());
    }

    let path = path_resolver::paths().app_local_data(&name).unwrap();
    if path.exists() {
        Ok(SET.into())
    } else {
        Ok(UNSET.into())
    }
}

#[tauri::command]
pub async fn list_downloaded_models(
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<Vec<String>, String> {
    let ctx = &mut app_state.lock().await;
    let cfg = &ctx.config;
    let mut found = vec![];
    for (name, _) in cfg.get_available_models() {
        let p = path_resolver::paths().app_local_data(&name).unwrap();
        if p.exists() {
            found.push(name.to_string());
        }
    }
    Ok(found)
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
        let ctx = &mut app_state.lock().await;
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

    let result = tauri::async_runtime::spawn_blocking(move || {
        models::service::fetch_model(&url, &model_name, &path, window)
    })
    .await
    .map_err(|e| e.to_string())?;

    result.map_err(|e| format!("Model fetch failed: {:?}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_model(
    model_name: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<(), String> {
    let ctx = &mut app_state.lock().await;
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

        infrastructure::service::set_config("model_name".into(), "".into())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn set_default_model(
    model_name: String,
    app_state: State<'_, Arc<Mutex<Context>>>,
) -> Result<(), String> {
    let mut ctx = app_state.lock().await;

    inference::service::activate_model(model_name, &mut ctx).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_user_model(model_name: String, model_path: String) -> Result<String, String> {
    let path = PathBuf::from(&model_path);

    if !path.exists() {
        return Err("File does not exist.".to_string());
    }

    let backend = LlamaBackend::init().map_err(|e| format!("Backend init failed: {:?}", e))?;

    // 1. Tell llama.cpp to ONLY load the metadata/vocab, skipping the heavy weights
    let model_params = LlamaModelParams::default().with_vocab_only(true);

    // 2. Attempt to load. This acts as validation.
    // It will return an Err if the file is corrupted, not a GGUF,
    // or uses an architecture llama.cpp doesn't support.
    let _model = LlamaModel::load_from_file(&backend, path, &model_params)
        .map_err(|e| format!("Invalid or unsupported GGUF model: {:?}", e))?;

    // If we reach this line, the GGUF is perfectly valid for text inference.
    _ = infrastructure::service::save_user_model(model_name, model_path);
    Ok("Valid text model detected and verified!".to_string())
}

#[tauri::command]
pub async fn get_user_models() -> Result<HashMap<String, String>, String> {
    let result = infrastructure::service::get_user_models()
        .map_err(|e| format!("Failed to get user models: {:?}", e))?;
    Ok(result)
}
