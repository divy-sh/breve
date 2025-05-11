use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, Special};
use llama_cpp_2::sampling::LlamaSampler;
use std::io::Write;
use tauri::{Emitter, Window};

use super::conversation::Conversation;

pub struct Inference {
    pub model: LlamaModel,
    pub backend: LlamaBackend,
}

impl Inference {
    pub fn init() -> Result<Inference, String> {
        let backend = LlamaBackend::init().unwrap();
        let model = LlamaModel::load_from_file(
            &backend,
            "./src/models/Llama-3.2-3B-Instruct-Q4_K_L.gguf",
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
            .map_err(|e| format!("Session creation error: {:?}", e))?;

        let system_prompt = "You are an AI assistant named Breve, designed to respond to user queries efficiently and accurately.";
        let mut formatted_prompt = format!(
            "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n{}\n<|eot_id|>",
            system_prompt
        );

        for msg in &conv.body {
            formatted_prompt.push_str(&format!(
                "<|start_header_id|>{}<|end_header_id|>\n\n{}\n<|eot_id|>",
                msg.role, msg.content
            ));
        }

        formatted_prompt.push_str("<|start_header_id|>assistant<|end_header_id|>\n\n");
        let mut tokens_list = self
            .model
            .str_to_token(&formatted_prompt, AddBos::Always)
            .map_err(|e| format!("Tokenization failed: {:?}", e))?;

        let max_ctx: i32 = 2048;
        let max_output_tokens = 512;
        let max_input_tokens = max_ctx.saturating_sub(max_output_tokens);

        if tokens_list.len() as i32 > max_input_tokens {
            tokens_list = tokens_list[tokens_list.len() - max_input_tokens as usize..].to_vec();
            eprintln!(
                "Prompt was too long ({} tokens); trimmed to {} tokens.",
                formatted_prompt.len(),
                tokens_list.len()
            );
        }

        let mut batch = LlamaBatch::new(512, 8);
        let last_index = tokens_list.len() as i32 - 1;
        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            let is_last = i == last_index;
            batch.add(token, i, &[0], is_last).map_err(|e| {
            format!("Batch size full: {:?}", e)
        })?;
        }

        if let Err(e) = ctx.decode(&mut batch) {
            return Err(format!("Initial decode failed: {:?}", e));
        }

        let mut n_cur = batch.n_tokens();
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();
        let mut message = String::new();
        println!("prompt: {}",formatted_prompt);
        while n_cur <= max_ctx && (n_cur - batch.n_tokens()) < max_output_tokens {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            if token == self.model.token_eos() {
                eprintln!("End of sequence token reached.");
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
}