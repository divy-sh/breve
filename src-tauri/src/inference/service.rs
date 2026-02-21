use crate::{
    inference::models::Inference,
    infrastructure::{app::App, path_resolver},
    settings::models::Config,
    settings,
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

pub fn activate_model(model_name: String, app: &mut App) -> Result<(), String> {
    validate_model(&app.config, &model_name)?;
    // persist
    settings::service::set_config("model_name".into(), model_name.clone(), app)
        .map_err(|e| e.to_string())?;

    app.config.default_model = model_name.clone();

    initialize_inference(app);

    Ok(())
}

fn initialize_inference(app: &mut App) {
    match Inference::init(&app.config) {
        Ok(inference) => {
            app.inference = Some(inference);
        }
        Err(e) => eprintln!("Inference init failed: {}", e),
    }
}
