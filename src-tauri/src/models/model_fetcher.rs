use hf_hub::api::sync::Api;
use anyhow::{Context, Result};
use tauri::{Emitter, Window};
use std::path::Path;
use std::fs;

pub struct ModelFetcher;

impl ModelFetcher {
    /// Fetches a model file from the Hugging Face hub into `model_path`.
    /// If `model_path` already exists, the download is skipped.
    pub fn fetch_model(model_url: &str, model_name: &str, model_path: &str, window: Window) -> Result<()> {
        let dest_path = Path::new(&model_path);
        if dest_path.exists() {
            return Ok(());
        }
        let _ = window.emit("downloading-model", true);
        let api = Api::new().context("failed to build HF API client")?;

        // Get a reference to the model repository
        let repo = api.model(model_url.to_string());

        // Download the specified file to a temporary location
        let tmp_path = repo.get(&model_name).context("failed to download model file")?;

        println!("Downloaded model to temporary path: {:?}", tmp_path);
        
        // Ensure parent directory exists
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).context("failed to create model_path directory")?;
        }

        fs::copy(&tmp_path, &dest_path).and_then(|_| fs::remove_file(&tmp_path))
            .context("failed to move downloaded model to model_path")?;

        println!("Model downloaded to: {}", model_path);
        let _ = window.emit("downloading-model", false);
        Ok(())
    }
}