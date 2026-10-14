use crate::core::configuration::controller as configuration_controller;
use crate::core::infrastructure::{consts, controller as infrastructure_controller};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct ModelSettings {
    pub temperature: f32,
    pub system_prompt: String,
    pub max_output_length: u64,
    pub max_context_length: u64,
    pub min_output_length: u64,
    pub output_length_limit: u64,
    pub min_context_length: u64,
    pub context_length_limit: u64,
}

pub async fn set_config(key: String, value: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || infrastructure_controller::set_config(key, value))
        .await
        .map_err(|error| error.to_string())?
}

pub async fn get_config(key: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || infrastructure_controller::get_config(key))
        .await
        .map_err(|error| error.to_string())?
}

pub async fn get_model_config() -> Result<ModelSettings, String> {
    tokio::task::spawn_blocking(|| {
        let config = configuration_controller::get_model_config();
        ModelSettings {
            temperature: config.temperature,
            system_prompt: config.system_prompt,
            max_output_length: config.max_output_length,
            max_context_length: config.max_context_length,
            min_output_length: consts::MIN_OUTPUT_LENGTH,
            output_length_limit: consts::DEFAULT_MAX_OUTPUT_LENGTH,
            min_context_length: consts::MIN_CONTEXT_LENGTH.min(config.batch_size),
            context_length_limit: config.batch_size,
        }
    })
    .await
    .map_err(|error| error.to_string())
}

pub async fn set_model_config(settings: ModelSettings) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let payload = HashMap::from([
            ("temperature".to_string(), Value::from(settings.temperature)),
            (
                "system_prompt".to_string(),
                Value::from(settings.system_prompt),
            ),
            (
                "max_output_length".to_string(),
                Value::from(settings.max_output_length),
            ),
            (
                "max_context_length".to_string(),
                Value::from(settings.max_context_length),
            ),
        ]);
        configuration_controller::set_model_config(payload)
    })
    .await
    .map_err(|error| error.to_string())?
}
