use tauri::Window;

pub mod generate_text;

#[tauri::command]
fn greet(name: &str, window: Window) {
    let _ = generate_text::generate_handler(name.to_string(), window);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
