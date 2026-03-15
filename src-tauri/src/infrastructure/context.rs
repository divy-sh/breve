use crate::{configuration::models::Config, inference::models::Inference};

pub struct Context {
    pub config: Config,
    pub inference: Option<Inference>,
}

impl Context {
    pub fn init() -> Result<Context, String> {
        let config = Config::init();

        Ok(Context {
            config,
            inference: None,
        })
    }
}
