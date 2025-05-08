use tauri::Window;

pub mod models;
pub mod generate_text;

#[tauri::command]
async fn greet(name: &str, window: Window) -> Result<(), String> {
    let _ = generate_text::generate_handler(name.to_string(), window);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
