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
    pub inference: Option<Inference>,
}

impl ConversationController {
    pub fn new() -> Self {
        let config = Config::init().unwrap();
        let conversation_dao = ConversationDao::init(&config).unwrap();
        ConversationController {
            dao: conversation_dao,
            config: config,
            inference: None,
        }
    }

    pub fn set_inference(&mut self, inference: Inference) {
        self.inference = Some(inference);
    }

    pub fn set_model_name(&mut self, model_name: String) {
        self.config.model_name = model_name;
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

            // Ensure inference is available
            let inference = match self.inference.as_mut() {
                Some(i) => i,
                None => return Ok(None),
            };

            match inference.generate_text(&conversation, window) {
                Ok(ai_reply) => {
                    conversation.add_message("assistant", &ai_reply);
                    self.dao.update_conversation(&conversation)?;
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

    pub fn get_conversation(&self, id: &str) -> Result<Option<Conversation>> {
        self.dao.get_conversation(id)
    }

    pub fn delete_conversation(&self, id: &str) -> Result<String, Error> {
        self.dao.delete_conversation(id)
    }
}
