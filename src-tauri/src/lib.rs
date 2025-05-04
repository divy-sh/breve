use std::error::Error;
use std::path::Path;
use candle_core::Device;
use candle_core::quantized::gguf_file;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("{}", generate_text(name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_text(text: &str) -> String {
    let model_path = Path::new("models/SmolLM-1.7B-Instruct-v0.2-IQ3_XS.gguf");
    text.to_string()
}
