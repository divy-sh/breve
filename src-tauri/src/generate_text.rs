use std::io::{self, Write};

use llama_cpp::{standard_sampler::StandardSampler, LlamaModel, LlamaParams, SessionParams};

pub fn generate_handler(prompt: &str) -> String {
    let model = LlamaModel::load_from_file(
        "models/SmolLM-1.7B-Instruct-v0.2-IQ3_XS.gguf",
        LlamaParams::default(),
    )
    .expect("Could not load model");

    let mut ctx = model
        .create_session(SessionParams::default())
        .expect("Failed to create session");

    ctx.advance_context(prompt).unwrap();

    let max_tokens = 1024;
    let mut decoded_tokens = 0;

    let completions = ctx
        .start_completing_with(StandardSampler::default(), 1024)
        .expect("")
        .into_strings();

    for completion in completions {
        print!("{completion}");
        let _ = io::stdout().flush();

        decoded_tokens += 1;

        if decoded_tokens > max_tokens {
            break;
        }
    }
    return prompt.to_string();
}
