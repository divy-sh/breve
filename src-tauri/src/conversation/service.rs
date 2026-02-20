use std::fmt::Error;
use std::vec;

use crate::conversation::models::Conversation;
use crate::conversation::repository as dao;
use crate::infrastructure::app::App;

use rusqlite::Result;
use tauri::Window;
use uuid::Uuid;

pub fn get_conversation_ids(app: &mut App) -> Vec<String> {
    if let Ok(conversation_ids) = dao::get_conversation_ids(&app.db.conn) {
        return conversation_ids;
    } else {
        return vec![];
    }
}

pub fn start_new_conversation(title: &str, app: &mut App) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let conversation = Conversation::new(id.clone(), title.to_string());
    dao::add_conversation(&conversation, &app.db.conn)?;
    Ok(id)
}

pub fn continue_conversation(
    conv_id: &str,
    user_input: &str,
    window: Window,
    app: &mut App,
) -> Result<Option<String>> {
    if let Some(mut conversation) = dao::get_conversation(conv_id, &app.db.conn)? {
        conversation.add_message("user", user_input);

        // Ensure inference is available
        let inference = match app.inference.as_mut() {
            Some(i) => i,
            None => return Ok(None),
        };

        match inference.generate_text(&conversation, window) {
            Ok(ai_reply) => {
                conversation.add_message("assistant", &ai_reply);
                dao::update_conversation(&conversation, &app.db.conn)?;
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

pub fn get_conversation(id: &str, app: &mut App) -> Result<Option<Conversation>> {
    dao::get_conversation(id, &app.db.conn)
}

pub fn delete_conversation(id: &str, app: &mut App) -> Result<String, Error> {
    dao::delete_conversation(id, &app.db.conn)
}
