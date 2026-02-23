use crate::settings::service;

#[tauri::command]
pub async fn get_config(key: String) -> Result<String, String> {
    service::get_config(key).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_config(
    key: String,
    value: String,
) -> Result<(), String> {
    service::set_config(key, value).map_err(|e| e.to_string())
}
