use std::fmt::Error;

use crate::{config::config_handler::Config, models::conversation::{Conversation, Message}};
use rusqlite::{Connection, Result, params};
use serde_json;

pub struct ConversationDao {
    conn: Connection,
}

impl ConversationDao {
    pub fn init(config: &Config) -> Result<Self> {
        let conn = Connection::open(config.get_db_path())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                lastUpdated TEXT NOT NULL
            )",
            [],
        )?;
        Ok(ConversationDao { conn })
    }

    pub fn add_conversation(&self, conv: &Conversation) -> Result<()> {
        let body_json = serde_json::to_string(&conv.body)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        self.conn.execute(
            "INSERT INTO conversations (id, title, body, lastUpdated) VALUES (?1, ?2, ?3, DATETIME('now'))",
            params![conv.id, conv.title, body_json],
        )?;
        Ok(())
    }

    pub fn update_conversation(&self, conv: &Conversation) -> Result<()> {
        let body_json = serde_json::to_string(&conv.body)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        self.conn.execute(
            "UPDATE conversations SET title = ?1, body = ?2, lastUpdated = DATETIME('now') WHERE id = ?3",
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
        let mut stmt = self
            .conn
            .prepare("SELECT id, lastUpdated FROM conversations order by lastUpdated DESC")?;
        let ids = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(ids)
    }

    pub fn delete_conversation(&self, id: &str) -> Result<String, Error> {
        if let Ok(deleted) = self
            .conn
            .execute("DELETE FROM conversations where id = ?1", params![id])
        {
            if deleted <= 0 {
                return Err(Error);
            } else {
            }
        } else {
            return Err(Error);
        }
        return Ok("delete successful".to_string());
    }
}
