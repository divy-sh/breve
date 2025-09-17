use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, Special};
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;
use std::io::Write;
use std::num::NonZero;
use tauri::{Emitter, Window};

use super::conversation::Conversation;
use crate::config::config_handler::Config;
use crate::config::path_resolver::paths;

pub struct Inference {
    pub model: LlamaModel,
    pub backend: LlamaBackend,
    pub batch_size: i32,
    pub max_context_length: i32,
    pub max_output_length: i32,
}

impl Inference {
    pub fn init(config: &Config) -> Result<Inference, String> {
        let model_path = paths().resource(config.model.clone())?;
        let backend = LlamaBackend::init().unwrap();
        let model = LlamaModel::load_from_file(
            &backend,
            model_path.to_str().unwrap(),
            &LlamaModelParams::default(),
        )
        .map_err(|e| {
            eprintln!("Model load error: {:?}", e);
            format!("Model load error: {:?}", e)
        })?;

        return Ok(Inference {
            model: model,
            backend: backend,
            batch_size: config.batch_size,
            max_context_length: config.max_context_length,
            max_output_length: config.max_context_size,
        });
    }

    pub fn generate_text(&mut self, conv: &Conversation, window: Window) -> Result<String, String> {
        println!("max_context_length: {}, max_output_length: {}", self.max_context_length, self.max_output_length);
        let ctx_params = LlamaContextParams::default()
            .with_n_batch(self.batch_size.try_into().unwrap())
            .with_n_ctx(Some(
                NonZero::try_from(self.max_context_length as u32).unwrap(),
            ));
        let mut ctx = self
            .model
            .new_context(&self.backend, ctx_params)
            .map_err(|e| format!("Session creation error: {:?}", e))?;

        let mut batch = LlamaBatch::new(self.batch_size as usize, 8);
        let tokens_list = self.format_prompt(conv);
        let last_index = tokens_list.len() as i32 - 1;

        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            batch.add(token, i, &[0], i == last_index).unwrap();
        }

        if let Err(e) = ctx.decode(&mut batch) {
            return Err(format!("Initial decode failed: {:?}", e));
        }

        let mut n_cur = batch.n_tokens();
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();
        let mut message = String::new();
        while n_cur <= self.batch_size && (n_cur - batch.n_tokens()) <= self.max_output_length {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            if token == self.model.token_eos() {
                break;
            }
            let output_bytes = self.model.token_to_bytes(token, Special::Tokenize).unwrap();
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

            if let Err(e) = ctx.decode(&mut batch) {
                eprintln!("Decode failed at n_cur = {}: {:?}", n_cur, e);
                break;
            }
        }
        Ok(message)
    }

    pub fn format_prompt(&self, conv: &Conversation) -> Vec<LlamaToken> {
        let system_prompt = "You are a friendly AI assistant named Breve.
        You are designed to respond to user queries in a friendly and empathetic manner.
        Answer without making up facts or hallucinating.";
        let mut formatted_prompt = format!(
            "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n{}\n<|eot_id|>",
            system_prompt
        );
        let end_str = "<|start_header_id|>assistant<|end_header_id|>\n";
        let mut message_segments = Vec::new();
        let mut total_len = formatted_prompt.len() + end_str.len();

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

        message_segments.reverse();
        for segment in message_segments {
            formatted_prompt.push_str(&segment);
        }

        formatted_prompt.push_str(&end_str);
        self.model
            .str_to_token(&formatted_prompt, AddBos::Always)
            .map_err(|e| format!("Tokenization failed: {:?}", e))
            .unwrap()
    }
}
