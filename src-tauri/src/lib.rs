use std::sync::Arc;

pub mod conversation;
pub mod inference;
pub mod infrastructure;
pub mod models;
pub mod settings;

use tauri::{Manager, async_runtime::Mutex};

use crate::infrastructure::context::Context;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_iap::init())
        .setup(|app| {
            infrastructure::path_resolver::init_app_paths(app.handle().clone());

            let mut ctx = Context::init()?;

            let saved_model =
                settings::service::get_config("model_name".to_string(), &mut ctx)
                    .unwrap_or_default();

            if !saved_model.is_empty() {
                let _ = inference::service::activate_model(saved_model, &mut ctx);
            }

            let shared_ctx = Arc::new(Mutex::new(ctx));

            app.manage(shared_ctx);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            conversation::controller::start_conversation,
            conversation::controller::continue_conversation,
            conversation::controller::get_conversation_ids,
            conversation::controller::get_conversation,
            conversation::controller::delete_conversation,
            models::controller::get_model_status,
            models::controller::get_available_models,
            models::controller::list_downloaded_models,
            models::controller::download_model,
            models::controller::delete_model,
            models::controller::set_default_model,
            models::controller::get_default_model,
            settings::controller::get_config,
            settings::controller::set_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
