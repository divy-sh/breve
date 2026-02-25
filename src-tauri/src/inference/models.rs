use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::{LlamaChatMessage, LlamaModel};
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::sampling::LlamaSampler;
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
            .with_n_ctx(Some(NonZero::try_from(config.max_context_length as u32).unwrap()));

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

        let tokens_list = self.format_prompt(conv)?;
        let last_index = tokens_list.len() as i32 - 1;

        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            batch.add(token, i, &[0], i == last_index).unwrap();
        }

        self.ctx
            .decode(&mut batch)
            .map_err(|e| format!("Decode failed: {:?}", e))?;

        let mut n_cur: u64 = batch.n_tokens() as u64;
        let cur: u64 = batch.n_tokens() as u64;
        let decoder: &mut encoding_rs::Decoder = &mut encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();
        let mut message = String::new();

        while n_cur < self.batch_size && n_cur - cur < self.max_output_length {
            let token = sampler.sample(&self.ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            if token == model.token_eos() {
                break;
            }

            let output_string = model.token_to_piece(token, decoder, true, None).unwrap();

            message += &output_string;
            let _ = window.emit(
                "llm-stream",
                StreamingContent {
                    id: conv.id.clone(),
                    content: output_string.clone(),
                },
            );
            batch.clear();
            batch.add(token, n_cur as i32, &[0], true).unwrap();
            n_cur += 1;

            if let Err(_) = self.ctx.decode(&mut batch) {
                break;
            }
        }
        Ok(message)
    }

    pub fn format_prompt(&self, conv: &Conversation) -> Result<Vec<llama_cpp_2::token::LlamaToken>, String> {
        let system_msg = LlamaChatMessage::new("system".to_string(), self.system_prompt.clone())
            .map_err(|e| format!("Invalid system prompt: {:?}", e))?;

        // 1. Start with all current messages
        let mut body_messages: Vec<LlamaChatMessage> = conv.body.iter().map(|msg| {
            let role = if msg.role == "user" { "user" } else { "assistant" };
            LlamaChatMessage::new(role.to_string(), msg.content.clone()).unwrap()
        }).collect();

        let template = self.model.chat_template(None)
            .map_err(|e| format!("Failed to get chat template: {:?}", e))?;

        let mut tokens: Vec<llama_cpp_2::token::LlamaToken>;
        let reserve_for_output = self.max_output_length as usize;
        
        // 2. Sliding Window: Remove oldest messages until the prompt fits
        // We loop, checking if (System + Body + New Output) <= Context Limit
        loop {
            let mut current_chat = vec![system_msg.clone()];
            current_chat.extend(body_messages.clone());

            let chat_str = self.model
                .apply_chat_template(&template, &current_chat, true)
                .map_err(|e| format!("Template error: {:?}", e))?;

            tokens = self.model
                .str_to_token(&chat_str, llama_cpp_2::model::AddBos::Never)
                .map_err(|e| format!("Tokenization error: {:?}", e))?;

            // Check if we are within bounds
            if tokens.len() + reserve_for_output <= self.max_context_length as usize {
                break;
            }

            // If too long and we have messages to remove, remove the oldest one (index 0)
            if !body_messages.is_empty() {
                body_messages.remove(0);
            } else {
                // If even the system prompt alone is too long, we must truncate the tokens directly
                tokens.truncate(self.max_context_length as usize - reserve_for_output);
                break;
            }
        }

        Ok(tokens)
    }
}

unsafe impl Send for Inference {}
unsafe impl Sync for Inference {}
