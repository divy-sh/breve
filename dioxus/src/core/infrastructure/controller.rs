use crate::core::infrastructure::service;

/// Reads a persisted app setting. Returns an error if `key` has never been set.
pub fn get_config(key: String) -> Result<String, String> {
    service::get_config(key)
}

/// Persists an app setting, overwriting any existing value for `key`.
pub fn set_config(key: String, value: String) -> Result<(), String> {
    service::set_config(key, value)
}
