use crate::core::{conversation::service, infrastructure::context::Context};
use crate::types::conversation::Conversation;

pub fn start_new_conversation(title: &str) -> Result<String, String> {
    service::start_new_conversation(title).map_err(|e| e.to_string())
}

/// Fetches all full conversations using `get_conversation_ids` and `get_conversation`
pub fn get_all_conversations() -> Result<Vec<Conversation>, String> {
    let ids = service::get_conversation_ids();
    let mut conversations = Vec::new();

    for id in ids {
        if let Ok(Some(conv)) = service::get_conversation(&id) {
            conversations.push(conv);
        }
    }

    Ok(conversations)
}

pub fn continue_conversation(
    mut conv_id: String,
    user_input: String,
    on_token: impl FnMut(&str) + Send + 'static,
) -> Result<String, String> {
    let mut ctx = Context::global().lock().map_err(|e| e.to_string())?;
    if conv_id.is_empty() {
        conv_id = service::start_new_conversation(&user_input).map_err(|e| e.to_string())?;
    }
    service::continue_conversation(&conv_id, &user_input, &mut ctx, on_token)
        .map_err(|e| e.to_string())
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
