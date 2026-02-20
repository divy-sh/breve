use std::sync::{Arc, Mutex};

pub mod conversation;
pub mod inference;
pub mod infrastructure;
pub mod models;
pub mod settings;

use conversation::service::ConversationController;
use settings::service::SettingsController;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_iap::init())
        .setup(|app| {
            infrastructure::path_resolver::init_app_paths(app.handle().clone());

            app.manage(Arc::new(Mutex::new(ConversationController::new())));
            app.manage(Arc::new(Mutex::new(SettingsController::new())));

            let settings_state = app.state::<Arc<Mutex<SettingsController>>>();
            let convo_state = app.state::<Arc<Mutex<ConversationController>>>();

            let saved_model = {
                let settings_lock = settings_state.lock().ok();
                settings_lock.and_then(|s| s.get_config("model_name".to_string()).ok())
            };

            if let Some(model_name) = saved_model {
                let _ = inference::service::activate_model(
                    model_name,
                    Arc::clone(settings_state.inner()),
                    Arc::clone(convo_state.inner()),
                );
            }

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
