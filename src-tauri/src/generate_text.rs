use llama_cpp::{standard_sampler::StandardSampler, LlamaModel, LlamaParams, SessionParams};
use std::io::{self, Write};
use tauri::{Emitter, Window};

pub fn generate_handler(prompt: String, window: Window) -> Result<(), String> {
    let model = LlamaModel::load_from_file(
        "models/smollm2-1.7b-instruct-q4_k_m.gguf",
        LlamaParams::default(),
    )
    .map_err(|e| {
        eprintln!("Model load error: {:?}", e);
        format!("Model load error: {:?}", e)
    })?;

    let mut ctx = model
        .create_session(SessionParams::default())
        .map_err(|e| {
            eprintln!("Session creation error: {:?}", e);
            format!("Session creation error: {:?}", e)
        })?;

    let formatted_prompt = format!(
        "<|im_start|>system\nYou are a helpful AI assistant named Breve<|im_end|>\n
        <|im_start|>user\n{}\n<|im_end|>\n
        <|im_start|>assistant\n",
        prompt
    );
    ctx.advance_context(&formatted_prompt).map_err(|e| {
        eprintln!("Advance context error: {:?}", e);
        format!("Advance context error: {:?}", e)
    })?;

    let max_tokens = 1024;
    let mut decoded_tokens = 0;

    let completions = ctx
        .start_completing_with(StandardSampler::default(), max_tokens)
        .map_err(|e| {
            eprintln!("Completion start error: {:?}", e);
            format!("Completion start error: {:?}", e)
        })?
        .into_strings();

    for chunk in completions {
        decoded_tokens += 1;
        if decoded_tokens > max_tokens {
            break;
        }
        if let Err(e) = window.emit("llm-stream", chunk.clone()) {
            eprintln!("Emit error: {:?}", e);
        }

        let _ = io::stdout().flush();
    }

    Ok(())
}
