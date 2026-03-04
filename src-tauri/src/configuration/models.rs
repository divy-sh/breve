use std::collections::HashMap;

use crate::{
    infrastructure::{consts, path_resolver},
    models::models::Model,
};

#[derive(serde::Serialize, Clone)]
pub struct Config {
    pub default_model: String,
    pub batch_size: u64,
    pub max_context_length: u64,
    pub max_output_length: u64,
    pub system_prompt: String,
    pub temperature: f32,
    pub models: &'static HashMap<String, Model>,
}

impl Config {
    pub fn init() -> Config {
        let global_mem_bytes: u64 = consts::DEFAULT_GLOBAL_MEM_BYTES;
        let model_size_bytes: u64 = consts::DEFAULT_MODEL_SIZE_BYTES;
        let memory_for_context: u64 = global_mem_bytes.saturating_sub(model_size_bytes);
        let bytes_per_token: u64 = consts::DEFAULT_BYTES_PER_TOKEN;
        let max_context_tokens = memory_for_context / bytes_per_token;
        let temperature = consts::DEFAULT_TEMPERATURE;


        let batch_size = max_context_tokens.clamp(4096, 32768);
        Config {
            batch_size: batch_size,
            max_context_length: batch_size - consts::DEFAULT_MAX_OUTPUT_LENGTH,
            max_output_length: consts::DEFAULT_MAX_OUTPUT_LENGTH,
            system_prompt: consts::DEFAULT_SYSTEM_PROMPT.to_string(),
            models: consts::default_models(),
            default_model: "".to_string(),
            temperature: temperature,
        }
    }

    pub fn get_model_path(&self) -> String {
        path_resolver::paths()
            .app_local_data(&self.default_model)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_available_models(&self) -> &'static HashMap<String, Model> {
        &self.models
    }
}
