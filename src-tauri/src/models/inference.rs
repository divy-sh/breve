use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, Special};
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;
use std::collections::HashMap;
use std::io::Write;
use std::num::NonZero;
use std::sync::OnceLock;
use tauri::{Emitter, Window};

use super::conversation::Conversation;
use crate::config::config_handler::Config;

// Global Singleton to hold Model and Backend
struct GlobalLlama {
    backend: LlamaBackend,
    model: LlamaModel,
}

static LLAMA_GLOBAL: OnceLock<GlobalLlama> = OnceLock::new();

pub struct Inference {
    pub ctx: LlamaContext<'static>,
    pub batch_size: i32,
    pub max_context_length: i32,
    pub max_output_length: i32,
    pub model_attrs: HashMap<String, String>,
    pub system_prompt: String,
}

impl Inference {
    pub fn init(config: &Config) -> Result<Inference, String> {
        let global_resources = LLAMA_GLOBAL.get_or_init(|| {
            let backend = LlamaBackend::init().expect("Backend init failed");

            let model = LlamaModel::load_from_file(
                &backend,
                config.get_model_path(),
                &LlamaModelParams::default(),
            )
            .expect("Model load failed");

            GlobalLlama { backend, model }
        });

        let ctx_params = LlamaContextParams::default()
            .with_n_batch(config.batch_size.try_into().unwrap())
            .with_n_ctx(Some(
                NonZero::try_from((config.max_context_length + config.max_context_size) as u32)
                    .unwrap(),
            ));

        let ctx = global_resources
            .model
            .new_context(&global_resources.backend, ctx_params)
            .map_err(|e| format!("Session creation error: {:?}", e))?;

        Ok(Inference {
            ctx,
            batch_size: config.batch_size,
            max_context_length: config.max_context_length,
            max_output_length: config.max_context_size,
            model_attrs: config
                .get_available_models()
                .get(&config.model_name)
                .cloned()
                .unwrap_or_default(),
            system_prompt: config.system_prompt.clone(),
        })
    }

    pub fn generate_text(&mut self, conv: &Conversation, window: Window) -> Result<String, String> {
        let global = LLAMA_GLOBAL.get().ok_or("Global model not initialized")?;
        let model = &global.model;

        // Clear cache as we re-send full history formatted for the specific model
        self.ctx.clear_kv_cache();

        let mut batch = LlamaBatch::new(self.batch_size as usize, 8);

        let tokens_list = self.format_prompt(conv, model)?;
        let last_index = tokens_list.len() as i32 - 1;

        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            batch.add(token, i, &[0], i == last_index).unwrap();
        }

        if let Err(e) = self.ctx.decode(&mut batch) {
            return Err(format!("Initial decode failed: {:?}", e));
        }

        let mut n_cur = batch.n_tokens();
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();
        let mut message = String::new();

        while n_cur <= (self.max_context_length + self.max_output_length)
            && (n_cur - batch.n_tokens()) <= self.max_output_length
        {
            let token = sampler.sample(&self.ctx, batch.n_tokens() - 1);
            sampler.accept(token);

            if token == model.token_eos() {
                break;
            }

            let output_bytes = model.token_to_bytes(token, Special::Tokenize).unwrap();
            let mut output_string = String::with_capacity(32);
            let _ = decoder.decode_to_string(&output_bytes, &mut output_string, false);

            message += &output_string;

            if let Err(e) = window.emit("llm-stream", output_string.clone()) {
                eprintln!("Emit error: {:?}", e);
            }

            let _ = std::io::stdout().flush();
            batch.clear();

            if let Err(e) = batch.add(token, n_cur, &[0], true) {
                eprintln!("Batch add error: {:?}", e);
                break;
            }

            n_cur += 1;

            if let Err(e) = self.ctx.decode(&mut batch) {
                eprintln!("Decode failed at n_cur = {}: {:?}", n_cur, e);
                break;
            }
        }
        Ok(message)
    }

    // Updated signature to accept &LlamaModel
    pub fn format_prompt(
        &self,
        conv: &Conversation,
        model: &LlamaModel,
    ) -> Result<Vec<LlamaToken>, String> {
        let prompt_start = self.model_attrs.get("prompt_start").cloned().unwrap();
        let prompt_end = self.model_attrs.get("prompt_end").cloned().unwrap();
        let sys = self.model_attrs.get("sys").cloned().unwrap();

        let mut formatted_prompt = prompt_start
            .replace("{role}", &sys)
            .replace("{message}", &self.system_prompt)
            + prompt_end.clone().as_str();
        let end_str = "<|start_header_id|>assistant<|end_header_id|>\n";
        let mut message_segments = Vec::new();
        let mut total_len = formatted_prompt.len() + end_str.len();

        // 4. Iterate backwards to respect context window
        for msg in conv.body.iter().rev() {
            let segment = format!(
                "<|start_header_id|>{}<|end_header_id|>\n{}\n<|eot_id|>",
                msg.role, msg.content
            );

            if total_len + segment.len() < self.max_context_length.try_into().unwrap() {
                total_len += segment.len();
                message_segments.push(segment);
            } else {
                break;
            }
        }

        // 5. Assemble and Tokenize
        message_segments.reverse();
        for segment in message_segments {
            formatted_prompt.push_str(&segment);
        }

        formatted_prompt.push_str(&end_str);

        model
            .str_to_token(&formatted_prompt, AddBos::Always)
            .map_err(|e| format!("Tokenization failed: {:?}", e))
    }
}

unsafe impl Send for Inference {}
unsafe impl Sync for Inference {}
