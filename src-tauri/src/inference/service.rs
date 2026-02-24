use crate::{
    inference::models::Inference,
    infrastructure::{context::Context, path_resolver},
    settings,
    settings::models::Config,
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

pub fn activate_model(model_name: String, ctx: &mut Context) -> Result<(), String> {
    validate_model(&ctx.config, &model_name)?;
    // persist
    settings::service::set_config("model_name".into(), model_name.clone())
        .map_err(|e| e.to_string())?;

    ctx.config.default_model = model_name.clone();

    initialize_inference(ctx);

    Ok(())
}

fn initialize_inference(ctx: &mut Context) {
    match Inference::init(&ctx.config) {
        Ok(inference) => {
            ctx.inference = Some(inference);
        }
        Err(e) => eprintln!("Inference init failed: {}", e),
    }
}
