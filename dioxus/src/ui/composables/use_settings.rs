use crate::core::infrastructure::controller as infrastructure_controller;

pub async fn set_config(key: String, value: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || infrastructure_controller::set_config(key, value))
        .await
        .map_err(|error| error.to_string())?
}
