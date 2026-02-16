use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub body: Vec<Message>,
}

impl Conversation {
    /// Create a new conversation
    pub fn new(id: String, title: String) -> Self {
        Conversation {
            id,
            title,
            body: Vec::new(),
        }
    }

    /// Add a message to the conversation
    pub fn add_message(&mut self, role: &str, content: &str) {
        self.body.push(Message {
            role: role.to_string(),
            content: content.to_string(),
        });
    }

    /// Get the last message (if any)
    pub fn get_last_message(&self) -> Option<&Message> {
        self.body.last()
    }

    /// Update the title of the conversation
    pub fn update_title(&mut self, new_title: &str) {
        self.title = new_title.to_string();
    }

    /// Get all messages from a specific role (e.g., "user")
    pub fn get_messages_by_role(&self, role: &str) -> Vec<&Message> {
        self.body.iter().filter(|msg| msg.role == role).collect()
    }

    /// Check if the conversation is empty
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    /// Clear all messages in the conversation
    pub fn clear_messages(&mut self) {
        self.body.clear();
    }
}
