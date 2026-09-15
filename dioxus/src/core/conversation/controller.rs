use crate::core::{
    conversation::{models::Conversation, service},
    infrastructure::context::Context,
};

pub fn start_conversation(title: String) -> Result<String, String> {
    service::start_new_conversation(&title).map_err(|e| e.to_string())
}

/// Continues a conversation, streaming generated tokens to `on_token` as
/// they're produced.
///
/// This performs blocking model inference, so callers should invoke it from
/// a background thread (e.g. `std::thread::spawn` or
/// `tokio::task::spawn_blocking`) rather than directly inside a UI event
/// handler, to avoid blocking rendering.
pub fn continue_conversation(
    conv_id: String,
    user_input: String,
    on_token: impl FnMut(&str),
) -> Result<Option<String>, String> {
    let mut ctx = Context::global().lock().map_err(|e| e.to_string())?;
    service::continue_conversation(&conv_id, &user_input, &mut ctx, on_token)
        .map_err(|e| format!("Inference failed: {:?}", e))
}

pub fn get_conversation_ids() -> Vec<String> {
    service::get_conversation_ids()
}

pub fn get_conversation(conv_id: String) -> Result<Option<Conversation>, String> {
    service::get_conversation(&conv_id).map_err(|e| e.to_string())
}

pub fn delete_conversation(conv_id: String) -> Result<String, String> {
    service::delete_conversation(&conv_id).map_err(|e| e.to_string())
}
