use crate::{
    inference::models::Inference,
    infrastructure::{consts, database::Database},
    settings::models::Config,
};

pub struct Context {
    pub config: Config,
    pub db: Database,
    pub inference: Option<Inference>,
}

impl Context {
    pub fn init() -> Result<Context, String> {
        let config = Config::init();
        let db_path = consts::get_db_path();
        let db = Database::new(&db_path);

        Ok(Context {
            config,
            db,
            inference: None,
        })
    }
}
