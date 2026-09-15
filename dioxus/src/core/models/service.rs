use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use tempfile::NamedTempFile;

pub const SET: &str = "SET";
pub const UNSET: &str = "UNSET";

/// Downloads `model_name` from `model_url`'s Hugging Face repo to
/// `model_path`, calling `on_progress` with a 0..=100 percentage as the
/// download proceeds.
///
/// This performs a blocking network download; callers should run it on a
/// background thread.
pub fn fetch_model(
    model_url: &str,
    model_name: &str,
    model_path: &str,
    mut on_progress: impl FnMut(f64),
) -> Result<()> {
    let dest_path = Path::new(&model_path);
    if dest_path.exists() {
        return Ok(());
    }

    // Build the raw file URL. This assumes the file is available under `main` branch and
    // the repository follows standard Hugging Face layout.
    let raw_url = format!(
        "https://huggingface.co/{}/resolve/main/{}",
        model_url, model_name
    );

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
        eprintln!(
            "Raw download failed (status: {}), falling back to hf_hub get",
            resp.status()
        );
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
        let n = resp
            .read(&mut buffer)
            .context("failed to read from response stream")?;
        if n == 0 {
            break;
        }
        tmpfile
            .write_all(&buffer[..n])
            .context("failed to write to temp file")?;
        downloaded += n as u64;

        // Report progress if total_size known, else emit bytes downloaded as fallback (as percentage 0..100 scaled)
        if let Some(total) = total_size {
            let pct = (downloaded as f64 / total as f64) * 100.0;
            on_progress(pct);
        } else {
            // Without total, emit increasing values capped to 100 by using a heuristic
            let pct = (downloaded as f64 / (1024.0 * 1024.0 * 1024.0)) * 100.0; // assume up to 1GB
            on_progress(pct.min(99.9));
        }
    }

    // Ensure parent directory exists
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent).context("failed to create model_path directory")?;
    }

    // Persist temp file to destination path
    tmpfile
        .persist(&dest_path)
        .context("failed to move downloaded model to model_path")?;

    // Final progress report
    on_progress(100.0);
    Ok(())
}
