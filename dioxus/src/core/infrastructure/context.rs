use std::sync::{Mutex, OnceLock};

use crate::core::{
    configuration::models::Config, inference::models::Inference, infrastructure::service,
};

/// Holds all shared, mutable application state: the current model
/// configuration and the (optional) currently-loaded inference session.
pub struct Context {
    pub config: Config,
    pub inference: Option<Inference>,
}

impl Context {
    fn new() -> Context {
        Context {
            config: Config::init(),
            inference: None,
        }
    }

    /// Returns the shared, lazily-initialized application context.
    ///
    /// On first access this also re-activates whichever model was
    /// previously selected (persisted in settings), mirroring the startup
    /// behaviour the old Tauri `setup` hook used to perform. Call
    /// [`crate::core::init`] during app startup if you want this to happen
    /// eagerly rather than on first use.
    pub fn global() -> &'static Mutex<Context> {
        static CONTEXT: OnceLock<Mutex<Context>> = OnceLock::new();
        CONTEXT.get_or_init(|| {
            let mut ctx = Context::new();

            let saved_model = service::get_config("model_name".to_string()).unwrap_or_default();
            if !saved_model.is_empty() {
                let _ = crate::core::inference::service::activate_model(saved_model, &mut ctx);
            }

            Mutex::new(ctx)
        })
    }
}
