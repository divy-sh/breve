use std::fs;
use std::path::{Path, PathBuf};

/// Directory name used to namespace this app's files under the OS's local
/// data directory, e.g. `~/Library/Application Support/com.breve.ai` on
/// macOS, `~/.local/share/com.breve.ai` on Linux, or
/// `%LOCALAPPDATA%\com.breve.ai` on Windows.
const APP_DIR_NAME: &str = "com.breve.ai";

fn app_local_data_dir() -> PathBuf {
    let base = dirs::data_local_dir()
        .or_else(dirs::data_dir)
        .or_else(dirs::home_dir)
        .expect("Could not determine a local data directory for this platform");
    base.join(APP_DIR_NAME)
}

/// Resolves `path` relative to this app's local data directory, creating
/// the directory (and any parents) if they don't already exist.
pub fn app_local_data<P: AsRef<Path>>(path: P) -> Result<PathBuf, String> {
    let dir = app_local_data_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Unable to create app local data directory: {}", e))?;
    }
    Ok(dir.join(path))
}
