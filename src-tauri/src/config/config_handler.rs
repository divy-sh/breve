use std::collections::{BTreeMap, HashMap};

use crate::config::path_resolver;

#[derive(Clone, Debug)]
pub struct Config {
    pub model_name: String,
    pub batch_size: i32,
    pub max_context_length: i32,
    pub max_output_length: i32,
    pub system_prompt: String,
    pub models: BTreeMap<String, HashMap<String, String>>,
    db_name: String,
}

impl Config {
    pub fn init() -> Result<Config, String> {
        // Disabling for now until we can better handle systems without OpenCL
        // let (global_mem_bytes, _) = Config::calculate_device_memory();

        let global_mem_bytes: u64 = 4 * 1024 * 1024 * 1024; // Assume 4GB for now
        // Memory usage estimates
        let model_size_bytes: u64 = 1024 * 1024 * 1024; //1GB quantized model
        let memory_for_context = global_mem_bytes.saturating_sub(model_size_bytes);

        // Approx memory per token estimate: 8KB
        let bytes_per_token: u64 = 8 * 1024;

        let max_context_tokens = (memory_for_context / bytes_per_token) as i32;

        // Cap at a sane upper bound (for stability)
        let max_tokens_clamped = max_context_tokens.clamp(4096, 32768);

        let max_output_length = 4096;
        let batch_size = max_tokens_clamped;

        let system_prompt = "You are a friendly AI assistant named Breve.
        You are designed to respond to user queries in a friendly and empathetic manner.
        Answer without making up facts or hallucinating.";

        let models = BTreeMap::from([
            (
                "gemma-3-1b-it-Q4_K_M.gguf".to_string(),
                HashMap::from([
                    ("repo".into(), "unsloth/gemma-3-1b-it-GGUF".into()),
                    ("prefix".into(), "<start_of_turn>".into()),
                    ("suffix".into(), "\n".into()),
                    ("eot".into(), "<end_of_turn>\n".into()),
                    ("sys".into(), "model".into()),
                    ("us".into(), "user".into()),
                    ("ast".into(), "model".into()),
                    ("supports_vision".into(), "true".into()),
                ]),
            ),
            (
                "Llama-3.2-1B-Instruct-Q4_K_M.gguf".to_string(),
                HashMap::from([
                    ("repo".into(), "bartowski/Llama-3.2-1B-Instruct-GGUF".into()),
                    ("prefix".into(), "<|start_header_id|>".into()),
                    ("suffix".into(), "<|end_header_id|>\n".into()),
                    ("eot".into(), "<|eot_id|>".into()),
                    ("sys".into(), "system".into()),
                    ("us".into(), "user".into()),
                    ("ast".into(), "assistant".into()),
                    ("supports_vision".into(), "false".into()),
                ]),
            ),
            (
                "SmolLM2-360M-Instruct.Q4_K_M.gguf".to_string(),
                HashMap::from([
                    ("repo".into(), "QuantFactory/SmolLM2-360M-Instruct-GGUF".into()),
                    ("prefix".into(), "<|im_start|>".into()),
                    ("suffix".into(), "\n".into()),
                    ("eot".into(), "<|im_end|>\n".into()),
                    ("sys".into(), "system".into()),
                    ("us".into(), "user".into()),
                    ("ast".into(), "assistant".into()),
                    ("supports_vision".into(), "false".into()),
                ]),
            ),
        ]);

        Ok(Config {
            model_name: "".to_string(),
            db_name: "data_store.sqlite".to_string(),
            batch_size: batch_size,
            max_context_length: max_tokens_clamped - max_output_length,
            max_output_length: max_output_length,
            system_prompt: system_prompt.to_string(),
            models: models,
        })
    }

    pub fn get_model_path(&self) -> String {
        path_resolver::paths()
            .app_local_data(&self.model_name)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_db_path(&self) -> String {
        path_resolver::paths()
            .app_local_data(&self.db_name)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_available_models(&self) -> BTreeMap<String, HashMap<String, String>> {
        self.models.clone()
    }

    pub fn get_model_attr(&self, key: &str) -> String {
        self.models
            .get(&self.model_name)
            .and_then(|m| m.get(key))
            .cloned()
            .unwrap_or_default()
    }
}
