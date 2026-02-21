use crate::{
    inference::models::Inference,
    infrastructure::{consts, database::Database},
    settings::models::Config,
};

pub struct App {
    pub config: Config,
    pub db: Database,
    pub inference: Option<Inference>,
}

impl App {
    pub fn init() -> Result<App, String> {
        let config = Config::init();
        let db_path = consts::get_db_path();
        let db = Database::new(&db_path);

        Ok(App {
            config,
            db,
            inference: None,
        })
    }
}
