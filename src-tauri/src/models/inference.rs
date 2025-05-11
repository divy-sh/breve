use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::{AddBos, Special};
use llama_cpp_2::sampling::LlamaSampler;
use std::io::Write;
use tauri::{Emitter, Window};

use super::conversation::{Conversation, Message};

pub struct Inference {
    pub model: LlamaModel,
    pub backend: LlamaBackend,
}

impl Inference {
    pub fn init() -> Result<Inference, String> {
        let backend = LlamaBackend::init().unwrap();
        let model = LlamaModel::load_from_file(
            &backend,
            "./models/smollm2-1.7b-instruct-q4_k_m.gguf",
            &LlamaModelParams::default(),
        )
        .map_err(|e| {
            eprintln!("Model load error: {:?}", e);
            format!("Model load error: {:?}", e)
        })?;
        return Ok(Inference {
            model: model,
            backend: backend,
        });
    }

    pub fn generate_text(&mut self, conv: &Conversation, window: Window) -> Result<String, String> {
        let ctx_params = LlamaContextParams::default();
        let mut ctx = self
            .model
            .new_context(&self.backend, ctx_params)
            .map_err(|e| {
                eprintln!("Session creation error: {:?}", e);
                format!("Session creation error: {:?}", e)
            })?;

        let mut formatted_prompt = String::from(
            "<|im_start|>system\nYou are an AI chatbot, 
        respond to user's queries efficiently. Do not hallucinate<|im_end|>\n",
        );

        let recent_messages: Vec<&Message> = conv
            .body
            .iter()
            .rev()
            .take(2)
            .collect::<Vec<&Message>>()
            .into_iter()
            .rev()
            .collect();
        for msg in recent_messages {
            formatted_prompt.push_str(&format!(
                "<|im_start|>{}\n{}<|im_end|>\n",
                msg.role, msg.content
            ));
        }

        formatted_prompt.push_str("<|im_start|>assistant\n");
        let tokens_list = self
            .model
            .str_to_token(&formatted_prompt, AddBos::Always)
            .map_err(|e| {
                eprintln!("Advance context error: {:?}", e);
                format!("Advance context error: {:?}", e)
            })?;

        let n_len = 1024;
        let mut batch = LlamaBatch::new(512, 1);
        let last_index = tokens_list.len() as i32 - 1;
        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            let is_last = i == last_index;
            batch.add(token, i, &[0], is_last).unwrap();
        }

        ctx.decode(&mut batch).expect("llama_decode() failed");
        let mut n_cur = batch.n_tokens();
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();
        let mut message = String::new();

        while n_cur <= n_len {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            if token == self.model.token_eos() {
                eprintln!();
                break;
            }
            let output_bytes = self.model.token_to_bytes(token, Special::Tokenize).unwrap();
            let mut output_string = String::with_capacity(32);
            let _decode_result = decoder.decode_to_string(&output_bytes, &mut output_string, false);
            message += &output_string;
            if let Err(e) = window.emit("llm-stream", output_string.clone()) {
                eprintln!("Emit error: {:?}", e);
            }
            let _ = std::io::stdout().flush();
            batch.clear();
            batch.add(token, n_cur, &[0], true).unwrap();
            n_cur += 1;
            ctx.decode(&mut batch).expect("failed to eval");
        }
        Ok(message)
    }
}
