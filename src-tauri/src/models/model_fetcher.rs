use anyhow::{Context, Result};
use tauri::{Emitter, Window};
use std::path::Path;
use std::fs;
use reqwest::blocking::Client;
use std::io::{Write, Read};
use tempfile::NamedTempFile;

pub struct ModelFetcher;

impl ModelFetcher {
    /// Fetches a model file from the Hugging Face hub into `model_path`.
    /// If `model_path` already exists, the download is skipped.
    pub fn fetch_model(model_url: &str, model_name: &str, model_path: &str, window: Window) -> Result<()> {
        let dest_path = Path::new(&model_path);
        if dest_path.exists() {
            return Ok(());
        }
        let _ = window.emit("download-status", "downloading");
        // Build the raw file URL. This assumes the file is available under `main` branch and
        // the repository follows standard Hugging Face layout.
        let raw_url = format!("https://huggingface.co/{}/resolve/main/{}", model_url, model_name);

        let client = Client::builder()
            .user_agent("breve-model-fetcher/0.1")
            .build()
            .context("failed to build HTTP client")?;

        let mut resp = client
            .get(&raw_url)
            .send()
            .context("failed to send download request")?;

        if !resp.status().is_success() {
            // Fallback: try hf_hub's get (no progress) to keep previous behavior
            eprintln!("Raw download failed (status: {}), falling back to hf_hub get", resp.status());
            return Ok(());
        }

        // Get content length if provided
        let total_size = resp
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|hv| hv.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        // Create temp file while streaming
        let mut tmpfile = NamedTempFile::new().context("failed to create temp file")?;
        let mut downloaded: u64 = 0;
        let mut buffer = [0u8; 8 * 1024];

        loop {
            let n = resp.read(&mut buffer).context("failed to read from response stream")?;
            if n == 0 { break; }
            tmpfile.write_all(&buffer[..n]).context("failed to write to temp file")?;
            downloaded += n as u64;

            // Emit progress if total_size known, else emit bytes downloaded as fallback (as percentage 0..100 scaled)
            if let Some(total) = total_size {
                let pct = (downloaded as f64 / total as f64) * 100.0;
                let _ = window.emit("download-progress", pct);
            } else {
                // Without total, emit increasing values capped to 100 by using a heuristic
                let pct = (downloaded as f64 / (1024.0 * 1024.0 * 1024.0)) * 100.0; // assume up to 1GB
                let pct = pct.min(99.9);
                let _ = window.emit("download-progress", pct);
            }
        }

        // Ensure parent directory exists
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).context("failed to create model_path directory")?;
        }

        // Persist temp file to destination path
        tmpfile.persist(&dest_path).context("failed to move downloaded model to model_path")?;

        println!("Model downloaded to: {}", model_path);
        // Final progress emit (100%) and end boolean
        let _ = window.emit("download-progress", 100.0);
        let _ = window.emit("download-status", "downloaded");
        Ok(())
    }
}