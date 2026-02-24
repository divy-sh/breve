use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, Special};
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;
use std::num::NonZero;
use std::sync::Arc;
use tauri::{Emitter, Window};

use crate::conversation::models::Conversation;
use crate::models::models::Model;
use crate::settings::models::Config;

#[derive(Debug, Clone, serde::Serialize)]
pub struct StreamingContent {
    pub id: String,
    pub content: String,
}

pub struct Inference {
    model: Arc<LlamaModel>,
    pub ctx: LlamaContext<'static>,
    pub batch_size: u64,
    pub max_context_length: u64,
    pub max_output_length: u64,
    pub model_attrs: Model,
    pub system_prompt: String,
}

impl Inference {
    pub fn init(config: &Config) -> Result<Self, String> {
        let backend =
            Arc::new(LlamaBackend::init().map_err(|e| format!("Backend init failed: {:?}", e))?);
        let model = Arc::new(
            LlamaModel::load_from_file(
                &backend,
                config.get_model_path(),
                &LlamaModelParams::default(),
            )
            .map_err(|e| format!("Model load failed: {:?}", e))?,
        );

        let ctx_params = LlamaContextParams::default()
            .with_n_batch(config.batch_size.try_into().unwrap())
            .with_n_ctx(Some(NonZero::try_from(config.batch_size as u32).unwrap()));

        let ctx = unsafe {
            let internal_ctx = model
                .new_context(&backend, ctx_params)
                .map_err(|e| format!("Session creation error: {:?}", e))?;
            std::mem::transmute::<LlamaContext<'_>, LlamaContext<'static>>(internal_ctx)
        };

        Ok(Self {
            model,
            ctx,
            batch_size: config.batch_size,
            max_context_length: config.max_context_length,
            max_output_length: config.max_output_length,
            model_attrs: config
                .get_available_models()
                .get(&config.default_model)
                .cloned()
                .unwrap(),
            system_prompt: config.system_prompt.clone(),
        })
    }

    pub fn generate_text(&mut self, conv: &Conversation, window: Window) -> Result<String, String> {
        let model = &self.model;
        self.ctx.clear_kv_cache();

        let mut batch = LlamaBatch::new(self.batch_size as usize, 8);

        let tokens_list = self.format_prompt(conv, model)?;
        let last_index = tokens_list.len() as i32 - 1;

        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            batch.add(token, i, &[0], i == last_index).unwrap();
        }

        self.ctx
            .decode(&mut batch)
            .map_err(|e| format!("Decode failed: {:?}", e))?;

        let mut n_cur: u64 = batch.n_tokens() as u64;
        let cur: u64 = batch.n_tokens() as u64;
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();
        let mut message = String::new();

        while n_cur < self.batch_size && n_cur - cur < self.max_output_length {
            let token = sampler.sample(&self.ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            if token == model.token_eos() {
                break;
            }

            let output_bytes = model.token_to_bytes(token, Special::Tokenize).unwrap();
            let mut output_string = String::with_capacity(32);
            _ = decoder.decode_to_string(&output_bytes, &mut output_string, false);

            message += &output_string;
            let _ = window.emit("llm-stream", StreamingContent {
                id: conv.id.clone(),
                content: output_string.clone(),
            });
            batch.clear();
            batch.add(token, n_cur as i32, &[0], true).unwrap();
            n_cur += 1;

            if let Err(_) = self.ctx.decode(&mut batch) {
                break;
            }
        }
        Ok(message)
    }

    pub fn format_prompt(
        &self,
        conv: &Conversation,
        model: &LlamaModel,
    ) -> Result<Vec<LlamaToken>, String> {
        let prefix = &self.model_attrs.prefix;
        let suffix = &self.model_attrs.suffix;
        let eot = &self.model_attrs.eot;
        let sys_role = &self.model_attrs.sys;
        let user_role = &self.model_attrs.us;
        let ast_role = &self.model_attrs.ast;

        let mut full_prompt = format!(
            "{}{}{}{}{}",
            prefix, sys_role, suffix, self.system_prompt, eot
        );
        let mut message_segments = Vec::new();
        let mut current_len = full_prompt.len();

        for msg in conv.body.iter().rev() {
            let role = if msg.role == "user" {
                user_role
            } else {
                ast_role
            };
            let segment = format!("{}{}{}{}{}", prefix, role, suffix, msg.content, eot);

            if current_len + segment.len() < self.max_context_length as usize {
                current_len += segment.len();
                message_segments.push(segment);
            } else {
                break;
            }
        }

        message_segments.reverse();
        for seg in message_segments {
            full_prompt.push_str(&seg);
        }
        full_prompt.push_str(&format!("{}{}{}", prefix, ast_role, suffix));

        model
            .str_to_token(&full_prompt, AddBos::Always)
            .map_err(|e| format!("{:?}", e))
    }
}

unsafe impl Send for Inference {}
unsafe impl Sync for Inference {}
