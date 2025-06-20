use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tauri::{AppHandle, Manager, path::BaseDirectory};

// Global path provider
pub struct AppPaths {
    app_handle: AppHandle,
}

impl AppPaths {
    pub fn resource<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, String> {
        self.app_handle
            .path()
            .resolve(path, BaseDirectory::Resource)
            .map_err(|e| format!("Failed to resolve resource path: {}", e))
    }

    pub fn config<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, String> {
        self.app_handle
            .path()
            .resolve(path, BaseDirectory::Config)
            .map_err(|e| format!("Failed to resolve config path: {}", e))
    }

    pub fn app_local_data<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, String> {
        self.app_handle
            .path()
            .resolve(path, BaseDirectory::AppLocalData)
            .map_err(|e| format!("Failed to resolve app local data path: {}", e))
    }
}

static APP_PATHS: OnceLock<AppPaths> = OnceLock::new();

pub fn init_app_paths(app_handle: AppHandle) {
    let _ = APP_PATHS.set(AppPaths { app_handle });
}

pub fn paths() -> &'static AppPaths {
    APP_PATHS.get().expect("App paths not initialized")
}
