use std::fmt::Error;
use std::vec;

use crate::core::conversation::models::Conversation;
use crate::core::conversation::repository as dao;
use crate::core::infrastructure::context::Context;

use rusqlite::Result;
use uuid::Uuid;

pub fn get_conversation_ids() -> Vec<String> {
    if let Ok(conversation_ids) = dao::get_conversation_ids() {
        return conversation_ids;
    } else {
        return vec![];
    }
}

pub fn start_new_conversation(title: &str) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let conversation = Conversation::new(id.clone(), title.to_string());
    dao::add_conversation(&conversation)?;
    Ok(id)
}

/// Continues an existing conversation by generating an assistant reply.
///
/// `on_token` is called with each chunk of generated text as it streams in,
/// so callers (e.g. the UI layer) can render partial output live.
pub fn continue_conversation(
    conv_id: &str,
    user_input: &str,
    ctx: &mut Context,
    mut on_token: impl FnMut(&str),
) -> Result<Option<String>> {
    if let Some(mut conversation) = dao::get_conversation(conv_id)? {
        conversation.add_message("user", user_input);

        // Ensure inference is available
        let inference = match ctx.inference.as_mut() {
            Some(i) => i,
            None => return Ok(None),
        };

        match inference.generate_text(&conversation, &mut on_token) {
            Ok(ai_reply) => {
                conversation.add_message("assistant", &ai_reply);
                dao::update_conversation(&conversation)?;
                Ok(Some(ai_reply))
            }
            Err(e) => {
                eprintln!("AI generation error: {}", e);
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

pub fn get_conversation(id: &str) -> Result<Option<Conversation>> {
    dao::get_conversation(id)
}

pub fn delete_conversation(id: &str) -> Result<String, Error> {
    dao::delete_conversation(id)
}
