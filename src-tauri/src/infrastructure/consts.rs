use crate::{infrastructure::path_resolver, models::models::Model};
use std::{collections::HashMap, sync::OnceLock};

pub static DB_NAME: &str = "data_store.sqlite";

pub static DEFAULT_SYSTEM_PROMPT: &str = "You are a friendly AI assistant named Breve.
You are designed to respond to user queries in a friendly and empathetic manner.
Answer without making up facts or hallucinating.";

pub const DEFAULT_GLOBAL_MEM_BYTES: u64 = 4 * 1024 * 1024 * 1024;
pub const DEFAULT_MODEL_SIZE_BYTES: u64 = 1024 * 1024 * 1024;
pub const DEFAULT_BYTES_PER_TOKEN: u64 = 8 * 1024;
pub const DEFAULT_MAX_OUTPUT_LENGTH: u64 = 4096;

static DEFAULT_MODELS: OnceLock<HashMap<String, Model>> = OnceLock::new();

pub fn default_models() -> &'static HashMap<String, Model> {
    DEFAULT_MODELS.get_or_init(|| {
        HashMap::from([
            (
                "gemma-3-1b-it-Q4_K_M.gguf".to_string(),
                Model {
                    name: "Gemma-3-1B-It".to_string(),
                    repo: "unsloth/gemma-3-1b-it-GGUF".to_string(),
                    size: 1024 * 1024 * 1024,
                    prefix: "<start_of_turn>".to_string(),
                    suffix: "\n".to_string(),
                    eot: "<end_of_turn>\n".to_string(),
                    sys: "model".to_string(),
                    us: "user".to_string(),
                    ast: "model".to_string(),
                    supports_vision: true,
                    params: "1B".to_string(),
                },
            ),
            (
                "Llama-3.2-1B-Instruct-Q4_K_M.gguf".to_string(),
                Model {
                    name: "Llama-3.2-1B-Instruct".to_string(),
                    repo: "bartowski/Llama-3.2-1B-Instruct-GGUF".to_string(),
                    size: 1024 * 1024 * 1024,
                    prefix: "<|start_header_id|>".to_string(),
                    suffix: "<|end_header_id|>\n".to_string(),
                    eot: "<|eot_id|>".to_string(),
                    sys: "system".to_string(),
                    us: "user".to_string(),
                    ast: "assistant".to_string(),
                    supports_vision: false,
                    params: "1B".to_string(),
                },
            ),
            (
                "SmolLM2-360M-Instruct.Q4_K_M.gguf".to_string(),
                Model {
                    name: "SmolLM2-360M-Instruct".to_string(),
                    repo: "QuantFactory/SmolLM2-360M-Instruct-GGUF".to_string(),
                    size: 1024 * 1024 * 1024,
                    prefix: "<start_of_turn>".to_string(),
                    suffix: "\n".to_string(),
                    eot: "<end_of_turn>\n".to_string(),
                    sys: "system".to_string(),
                    us: "user".to_string(),
                    ast: "assistant".to_string(),
                    supports_vision: false,
                    params: "360M".to_string(),
                },
            ),
        ])
    })
}

pub fn get_db_path() -> String {
    path_resolver::paths()
        .app_local_data(DB_NAME)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}