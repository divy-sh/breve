use std::fmt::Error;
use std::vec;

use crate::config::config_handler::Config;
use crate::dao::conversation_dao::ConversationDao;
use crate::models::conversation::Conversation;
use crate::models::inference::Inference;

use rusqlite::Result;
use tauri::Window;
use uuid::Uuid;

pub struct ConversationController {
    pub dao: ConversationDao,
    pub config: Config,
    pub inference: Inference,
}

impl ConversationController {
    pub fn new() -> Self {
        let config = Config::init().unwrap();
        let conversation_dao = ConversationDao::init().unwrap();
        if let Ok(inference) = Inference::init(&config) {
            ConversationController {
                dao: conversation_dao,
                config: config,
                inference,
            }
        } else if let Err(e) = Inference::init(&config) {
            panic!("Failed to initialize Inference: {}", e);
        } else {
            unreachable!("This should never happen");
        }
    }

    pub fn get_conversation_ids(&self) -> Vec<String> {
        if let Ok(conversation_ids) = self.dao.get_conversation_ids() {
            return conversation_ids;
        } else {
            return vec![];
        }
    }

    pub fn start_new_conversation(&mut self, title: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let conversation = Conversation::new(id.clone(), title.to_string());
        self.dao.add_conversation(&conversation)?;
        Ok(id)
    }

    pub fn continue_conversation(
        &mut self,
        conv_id: &str,
        user_input: &str,
        window: Window,
    ) -> Result<Option<String>> {
        if let Some(mut conversation) = self.dao.get_conversation(conv_id)? {
            conversation.add_message("user", user_input);

            if let Ok(ai_reply) = self.inference.generate_text(&conversation, window) {
                conversation.add_message("assistant", &ai_reply);

                self.dao.update_conversation(&conversation)?;
                Ok(Some(ai_reply))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_conversation(&self, id: &str) -> Result<Option<Conversation>> {
        self.dao.get_conversation(id)
    }

    pub fn delete_conversation(&self, id: &str) -> Result<String, Error> {
        self.dao.delete_conversation(id)
    }
}
