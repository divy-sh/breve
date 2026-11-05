use std::collections::HashMap;

use serde_json::Value;

use crate::core::{
    configuration::{models::Config, repository},
    inference,
    infrastructure::context::Context,
};

/// Returns a snapshot of the current model configuration.
pub fn get_model_config() -> Config {
    let ctx = Context::global().lock().unwrap();
    ctx.config.clone()
}

/// Merges `payload` into the current configuration, persists the changed
/// keys, and re-activates the (possibly new) default model.
pub fn set_model_config(payload: HashMap<String, Value>) -> Result<(), String> {
    let mut ctx = Context::global().lock().map_err(|e| e.to_string())?;

    let mut current_config_json = serde_json::to_value(&ctx.config).map_err(|e| e.to_string())?;

    if let Some(obj) = current_config_json.as_object_mut() {
        for (key, value) in payload.clone() {
            obj.insert(key, value);
        }
    }

    let updated_config: Config = serde_json::from_value(current_config_json)
        .map_err(|e| format!("Validation failed: {}", e))?;

    for (key, val) in payload {
        let _ = repository::set_model_config(key, val.to_string());
    }
    ctx.config = updated_config;
    inference::service::activate_model(ctx.config.default_model.clone(), &mut ctx)
}
