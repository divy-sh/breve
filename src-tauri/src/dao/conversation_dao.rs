use crate::models::conversation::{Conversation, Message};
use rusqlite::{params, Connection, Result};
use serde_json;

pub struct ConversationDao {
    conn: Connection,
}

impl ConversationDao {
    pub fn init() -> Result<Self> {
        let conn = Connection::open("./data_store.sqlite")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL
            )",
            [],
        )?;
        Ok(ConversationDao { conn })
    }

    pub fn add_conversation(&self, conv: &Conversation) -> Result<()> {
        let body_json = serde_json::to_string(&conv.body)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        self.conn.execute(
            "INSERT INTO conversations (id, title, body) VALUES (?1, ?2, ?3)",
            params![conv.id, conv.title, body_json],
        )?;
        Ok(())
    }

    pub fn update_conversation(&self, conv: &Conversation) -> Result<()> {
        let body_json = serde_json::to_string(&conv.body)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        self.conn.execute(
            "UPDATE conversations SET title = ?1, body = ?2 WHERE id = ?3",
            params![conv.title, body_json, conv.id],
        )?;
        Ok(())
    }

    pub fn get_conversation(&self, id: &str) -> Result<Option<Conversation>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, body FROM conversations WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let body_json: String = row.get(2)?;
            let body: Vec<Message> = serde_json::from_str(&body_json).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            Ok(Some(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                body,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_conversation_ids(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT id FROM conversations")?;
        let ids = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(ids)
    }
}
