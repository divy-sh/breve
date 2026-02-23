use std::sync::OnceLock;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;

use crate::infrastructure::consts;

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Debug)]
pub struct Database {
    pub pool: Pool,
}

static DB_INSTANCE: OnceLock<Database> = OnceLock::new();

impl Database {
    pub fn get_db() -> &'static Database {
        DB_INSTANCE.get_or_init(|| {
            let manager = SqliteConnectionManager::file(consts::get_db_path());
            let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
            let db = Database { pool };

            _ = db.init_conversation_dao();
            _ = db.init_settings_dao();
            
            db
        })
    }

    pub fn init_conversation_dao(&self) -> Result<()> {
        let conn = self.pool.get().expect("Failed to get connection from pool");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                lastUpdated TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn init_settings_dao(&self) -> Result<()> {
        let conn = self.pool.get().expect("Failed to get connection from pool");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_conn(&self) -> r2d2::PooledConnection<SqliteConnectionManager> {
        self.pool.get().expect("Database pool exhausted")
    }
}
