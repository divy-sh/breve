use crate::{
    inference::models::Inference,
    infrastructure::{config::Config, database::Database},
};

pub struct App {
    pub config: Config,
    pub db: Database,
    pub inference: Option<Inference>,
}

impl App {
    pub fn init() -> Result<App, String> {
        let config = Config::init().map_err(|e| e.to_string())?;
        let db_path = config.get_db_path();
        let db = Database::new(&db_path);

        Ok(App {
            config,
            db,
            inference: None,
        })
    }
}
