use std::io::{self, Write};
use tauri::{Emitter, Window};
use llama_cpp::{standard_sampler::StandardSampler, LlamaModel, LlamaParams, SessionParams};

pub fn generate_handler(prompt: String, window: Window) -> Result<(), String> {
    let model = LlamaModel::load_from_file(
        "models/SmolLM-1.7B-Instruct-v0.2-IQ3_XS.gguf",
        LlamaParams::default(),
    )
    .map_err(|e| format!("Model load error: {:?}", e))?;

    let mut ctx = model
        .create_session(SessionParams::default())
        .map_err(|e| format!("Session creation error: {:?}", e))?;

    let formatted_prompt = format!(
        "<|system|>Do not return any text other than your response to the user.<|user|>\n{}\n<|assistant|>\n",
        prompt
    );
    ctx.advance_context(&formatted_prompt)
        .map_err(|e| format!("Advance context error: {:?}", e))?;

    let max_tokens = 1024;
    let mut decoded_tokens = 0;

    let completions = ctx
        .start_completing_with(StandardSampler::default(), max_tokens)
        .map_err(|e| format!("Completion start error: {:?}", e))?
        .into_strings();

    for chunk in completions {
        decoded_tokens += 1;
        if decoded_tokens > max_tokens {
            break;
        }
        print!("{}", chunk);
        if let Err(e) = window.emit("llm-stream", chunk.clone()) {
            eprintln!("Emit error: {:?}", e);
        }

        let _ = io::stdout().flush();
    }

    Ok(())
}
