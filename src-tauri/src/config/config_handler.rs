use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::config::path_resolver;

#[derive(Clone)]
pub struct Config {
    pub model_name: String,
    pub batch_size: i32,
    pub max_context_length: i32,
    pub max_context_size: i32,
    db_name: String,
    models: BTreeMap<String, HashMap<String, String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelTemplate {
    pub bos: String,
    pub eos: String,
    pub system_prefix: String,
    pub user_prefix: String,
    pub assistant_prefix: String,
    pub system_prompt: String,
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
        let max_tokens_clamped = max_context_tokens.clamp(512, 10240);

        let max_context_size = 2048;
        let batch_size = max_tokens_clamped;

        let models = BTreeMap::from([
            (
                "Llama-3.2-1B-Instruct-Q4_K_S.gguf".to_string(),
                HashMap::from([
                    ("repo".into(), "bartowski/Llama-3.2-1B-Instruct-GGUF".into()),

                    ("bos".into(), "<|begin_of_text|>".into()),
                    ("eos".into(), "<|eot_id|>".into()),
                    ("usr".into(), "<|start_header_id|>user<|end_header_id|>\n".into()),
                    ("ast".into(), "<|start_header_id|>assistant<|end_header_id|>\n".into()),
                    ("sys".into(), "<|start_header_id|>sys<|end_header_id|>\n".into()),
                ]),
            ),
            (
                "Llama-3.2-1B-Instruct-Q6_K_L.gguf".to_string(),
                HashMap::from([
                    ("repo".into(), "bartowski/Llama-3.2-1B-Instruct-GGUF".into()),

                    ("bos".into(), "<|begin_of_text|>".into()),
                    ("eos".into(), "<|eot_id|>".into()),
                    ("usr".into(), "<|start_header_id|>user<|end_header_id|>\n".into()),
                    ("ast".into(), "<|start_header_id|>assistant<|end_header_id|>\n".into()),
                    ("sys".into(), "<|start_header_id|>sys<|end_header_id|>\n".into()),
                ]),
            ),
            // (
            //     "SmolLM2-360M-Instruct.Q8_0.gguf".to_string(),
            //     HashMap::from([
            //         ("repo".into(), "QuantFactory/SmolLM2-360M-Instruct-GGUF".into()),

            //         ("bos".into(), "<|begin_of_text|>".into()),
            //         ("eos".into(), "<|eot_id|>".into()),
            //         ("usr".into(), "<|start_header_id|>user<|end_header_id|>\n".into()),
            //         ("ast".into(), "<|start_header_id|>assistant<|end_header_id|>\n".into()),
            //         ("sys".into(), "<|start_header_id|>sys<|end_header_id|>\n".into()),
            //     ]),
            // ),
            // (
            //     "SmolLM3-Q4_K_M.gguf".to_string(),
            //     HashMap::from([
            //         ("repo".into(), "ggml-org/SmolLM3-3B-GGUF".into()),

            //         ("bos".into(), "<|begin_of_text|>".into()),
            //         ("eos".into(), "<|eot_id|>".into()),
            //         ("usr".into(), "<|start_header_id|>user<|end_header_id|>\n".into()),
            //         ("ast".into(), "<|start_header_id|>assistant<|end_header_id|>\n".into()),
            //         ("sys".into(), "<|start_header_id|>sys<|end_header_id|>\n".into()),
            //     ]),
            // ),
        ]);

        Ok(Config {
            model_name: "".to_string(),
            db_name: "data_store.sqlite".to_string(),
            batch_size,
            max_context_length: max_tokens_clamped - max_context_size,
            max_context_size,
            models,
        })
    }

    pub fn get_max_context_size(&self) -> i32 {
        self.max_context_size
    }

    pub fn get_max_context_length(&self) -> i32 {
        self.max_context_length
    }

    pub fn get_batch_size(&self) -> i32 {
        self.batch_size
    }

    pub fn get_model_name(&self) -> String {
        self.model_name.clone()
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
