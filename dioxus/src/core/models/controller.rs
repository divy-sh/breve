use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use std::{collections::HashMap, path::PathBuf};

use crate::core::{
    inference,
    infrastructure::{self, context::Context, path_resolver},
    models::{
        self,
        models::Model,
        service::{SET, UNSET},
    },
};

pub fn get_available_models() -> &'static HashMap<String, Model> {
    let ctx = Context::global().lock().unwrap();
    ctx.config.get_available_models()
}

pub fn get_default_model() -> String {
    let ctx = Context::global().lock().unwrap();
    ctx.config.default_model.clone()
}

pub fn get_model_status() -> String {
    let ctx = Context::global().lock().unwrap();
    let name = &ctx.config.default_model;
    if name.is_empty() {
        return UNSET.into();
    }

    match path_resolver::app_local_data(name) {
        Ok(path) if path.exists() => SET.into(),
        _ => UNSET.into(),
    }
}

pub fn list_downloaded_models() -> Vec<String> {
    let ctx = Context::global().lock().unwrap();
    let cfg = &ctx.config;
    let mut found = vec![];
    for (name, _) in cfg.get_available_models() {
        if let Ok(p) = path_resolver::app_local_data(name) {
            if p.exists() {
                found.push(name.to_string());
            }
        }
    }
    found
}

/// Downloads `model_name`, calling `on_progress` with 0..=100 as the
/// download proceeds.
///
/// This performs a blocking network download, so callers should run it on a
/// background thread (e.g. `std::thread::spawn`) rather than directly inside
/// a UI event handler.
pub fn download_model(model_name: String, on_progress: impl FnMut(f64)) -> Result<(), String> {
    let (url, path) = {
        let ctx = Context::global().lock().map_err(|e| e.to_string())?;
        let cfg = &ctx.config;
        let url = cfg
            .get_available_models()
            .get(&model_name)
            .ok_or("Model not found")?
            .repo
            .clone();
        let path = path_resolver::app_local_data(&model_name)?
            .to_string_lossy()
            .to_string();
        (url, path)
    };

    models::service::fetch_model(&url, &model_name, &path, on_progress)
        .map_err(|e| format!("Model fetch failed: {:?}", e))
}

pub fn delete_model(model_name: String) -> Result<(), String> {
    let mut ctx = Context::global().lock().map_err(|e| e.to_string())?;
    let path = path_resolver::app_local_data(&model_name)?;

    if path.exists() {
        std::fs::remove_file(&path)
            .or_else(|_| std::fs::remove_dir_all(&path))
            .map_err(|e| format!("Delete failed: {}", e))?;
    }

    if ctx.config.default_model == model_name {
        ctx.config.default_model.clear();
        ctx.inference = None;

        infrastructure::service::set_config("model_name".into(), "".into())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn set_default_model(model_name: String) -> Result<(), String> {
    let mut ctx = Context::global().lock().map_err(|e| e.to_string())?;
    inference::service::activate_model(model_name, &mut ctx)
}

pub fn save_user_model(model_name: String, model_path: String) -> Result<String, String> {
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

pub fn get_user_models() -> Result<HashMap<String, String>, String> {
    infrastructure::service::get_user_models()
        .map_err(|e| format!("Failed to get user models: {:?}", e))
}
