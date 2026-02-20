use std::sync::{Arc, Mutex};

use crate::{
    conversation::service::ConversationController,
    inference::models::Inference,
    infrastructure::{config::Config, path_resolver},
    settings::service::SettingsController,
};

fn validate_model(config: &Config, model_name: &str) -> Result<(), String> {
    if !config.get_available_models().contains_key(model_name) {
        return Err("Model not available".into());
    }

    let path = path_resolver::paths().app_local_data(model_name).unwrap();

    if !path.exists() {
        return Err("Model not downloaded".into());
    }

    Ok(())
}

pub fn activate_model(
    model_name: String,
    settings: Arc<Mutex<SettingsController>>,
    state: Arc<Mutex<ConversationController>>,
) -> Result<(), String> {
    let cfg = {
        let controller = state.lock().map_err(|_| "Controller lock poisoned")?;
        controller.config.clone()
    };

    validate_model(&cfg, &model_name)?;
    // persist
    settings
        .lock()
        .map_err(|_| "Settings lock poisoned")?
        .set_config("model_name".into(), model_name.clone())
        .map_err(|e| e.to_string())?;

    // update controller
    {
        let mut controller = state.lock().map_err(|_| "Controller lock poisoned")?;

        if controller.config.model_name == model_name && controller.inference.is_some() {
            return Ok(());
        }

        controller.set_model_name(model_name.clone());
        controller.inference = None;
    }

    // init inference async
    let state_clone = Arc::clone(&state);
    std::thread::spawn(move || {
        let cfg = match state_clone.lock() {
            Ok(c) => c.config.clone(),
            Err(_) => return,
        };

        initialize_inference(cfg, Arc::clone(&state_clone));
    });

    Ok(())
}

fn initialize_inference(config: Config, state_arc: Arc<Mutex<ConversationController>>) {
    match Inference::init(&config) {
        Ok(inference) => {
            if let Ok(mut controller) = state_arc.lock() {
                controller.set_inference(inference);
            }
        }
        Err(e) => eprintln!("Inference init failed: {}", e),
    }
}
