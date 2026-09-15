pub mod configuration;
pub mod conversation;
pub mod inference;
pub mod infrastructure;
pub mod models;

/// Eagerly initializes core application state: the database, model
/// configuration, and (if one was previously selected) the active model.
///
/// This is safe to call multiple times; initialization only happens once.
/// Call it once during app startup, before rendering any UI that depends on
/// it, to avoid paying that cost lazily on first use.
pub fn init() {
    let _ = infrastructure::context::Context::global();
}
