use rusqlite::Result;

use crate::{config::config_handler::Config, dao::settings_dao::SettingsDao};

pub struct SettingsController {
    pub dao: SettingsDao,
}

impl SettingsController {
    pub fn new() -> Self {
        let config = Config::init().unwrap();
        let settings_dao = SettingsDao::init(&config).unwrap();
        SettingsController { dao: settings_dao }
    }

    pub fn get_config(&self, key: String) -> Result<String, String> {
        self.dao.get_config(key)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Config not found".to_string())
    }

    pub fn set_config(&self, key: String, value: String) -> Result<(), String> {
        self.dao.set_config(key, value)
            .map_err(|e| e.to_string())
    }
}