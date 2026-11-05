use crate::core::{
    configuration::models::Config,
    inference::models::Inference,
    infrastructure::{self, context::Context, path_resolver},
};

fn validate_model(config: &Config, model_name: &str) -> Result<(), String> {
    if !config.get_available_models().contains_key(model_name) {
        return Err("Model not available".into());
    }

    let path = path_resolver::app_local_data(model_name)?;

    if !path.exists() {
        return Err("Model not downloaded".into());
    }

    Ok(())
}

pub fn activate_model(model_name: String, ctx: &mut Context) -> Result<(), String> {
    validate_model(&ctx.config, &model_name)?;

    ctx.initialize_backend()?;

    let mut config = ctx.config.clone();
    config.default_model = model_name.clone();
    let inference = Inference::init(
        &config,
        ctx.backend
            .as_ref()
            .expect("backend initialized before model load"),
    )?;

    infrastructure::service::set_config("model_name".into(), model_name)
        .map_err(|e| e.to_string())?;

    ctx.config = config;
    ctx.inference = Some(inference);
    Ok(())
}
