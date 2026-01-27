//! SQLite database for history storage

use crate::error::{AppError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// History item stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub mode_key: String,
    pub audio_path: Option<String>,
    pub transcript_raw: String,
    pub output_final: String,
    pub stt_provider: String,
    pub stt_model: String,
    pub llm_provider: Option<String>,
    pub llm_model: Option<String>,
    pub duration_ms: u64,
    pub error: Option<String>,
}

/// Database manager
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create the database
    pub fn new(path: &PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS history_items (
                id TEXT PRIMARY KEY,
                created_at TEXT NOT NULL,
                mode_key TEXT NOT NULL,
                audio_path TEXT,
                transcript_raw TEXT NOT NULL,
                output_final TEXT NOT NULL,
                stt_provider TEXT NOT NULL,
                stt_model TEXT NOT NULL,
                llm_provider TEXT,
                llm_model TEXT,
                duration_ms INTEGER NOT NULL,
                error TEXT
            )",
            [],
        )?;

        // Create index for faster queries
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_created_at ON history_items(created_at DESC)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_mode_key ON history_items(mode_key)",
            [],
        )?;

        Ok(())
    }

    /// Insert a new history item
    pub fn insert_history(&self, item: &HistoryItem) -> Result<()> {
        self.conn.execute(
            "INSERT INTO history_items (
                id, created_at, mode_key, audio_path, transcript_raw, output_final,
                stt_provider, stt_model, llm_provider, llm_model, duration_ms, error
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                item.id,
                item.created_at.to_rfc3339(),
                item.mode_key,
                item.audio_path,
                item.transcript_raw,
                item.output_final,
                item.stt_provider,
                item.stt_model,
                item.llm_provider,
                item.llm_model,
                item.duration_ms as i64,
                item.error,
            ],
        )?;
        Ok(())
    }

    /// Get all history items (paginated)
    pub fn get_history(&self, limit: usize, offset: usize) -> Result<Vec<HistoryItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, created_at, mode_key, audio_path, transcript_raw, output_final,
                    stt_provider, stt_model, llm_provider, llm_model, duration_ms, error
             FROM history_items
             ORDER BY created_at DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let items = stmt
            .query_map(params![limit as i64, offset as i64], |row| {
                Ok(HistoryItem {
                    id: row.get(0)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(1)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    mode_key: row.get(2)?,
                    audio_path: row.get(3)?,
                    transcript_raw: row.get(4)?,
                    output_final: row.get(5)?,
                    stt_provider: row.get(6)?,
                    stt_model: row.get(7)?,
                    llm_provider: row.get(8)?,
                    llm_model: row.get(9)?,
                    duration_ms: row.get::<_, i64>(10)? as u64,
                    error: row.get(11)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(items)
    }

    /// Get a single history item by ID
    pub fn get_history_item(&self, id: &str) -> Result<Option<HistoryItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, created_at, mode_key, audio_path, transcript_raw, output_final,
                    stt_provider, stt_model, llm_provider, llm_model, duration_ms, error
             FROM history_items
             WHERE id = ?1",
        )?;

        let item = stmt
            .query_row(params![id], |row| {
                Ok(HistoryItem {
                    id: row.get(0)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(1)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    mode_key: row.get(2)?,
                    audio_path: row.get(3)?,
                    transcript_raw: row.get(4)?,
                    output_final: row.get(5)?,
                    stt_provider: row.get(6)?,
                    stt_model: row.get(7)?,
                    llm_provider: row.get(8)?,
                    llm_model: row.get(9)?,
                    duration_ms: row.get::<_, i64>(10)? as u64,
                    error: row.get(11)?,
                })
            })
            .ok();

        Ok(item)
    }

    /// Update a history item (for reprocessing)
    pub fn update_history(&self, item: &HistoryItem) -> Result<()> {
        self.conn.execute(
            "UPDATE history_items SET
                mode_key = ?2,
                output_final = ?3,
                llm_provider = ?4,
                llm_model = ?5,
                error = ?6
             WHERE id = ?1",
            params![
                item.id,
                item.mode_key,
                item.output_final,
                item.llm_provider,
                item.llm_model,
                item.error,
            ],
        )?;
        Ok(())
    }

    /// Delete a history item
    pub fn delete_history(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM history_items WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Get total count of history items
    pub fn get_history_count(&self) -> Result<usize> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM history_items", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Search history by text
    pub fn search_history(&self, query: &str, limit: usize) -> Result<Vec<HistoryItem>> {
        let search_pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, created_at, mode_key, audio_path, transcript_raw, output_final,
                    stt_provider, stt_model, llm_provider, llm_model, duration_ms, error
             FROM history_items
             WHERE transcript_raw LIKE ?1 OR output_final LIKE ?1
             ORDER BY created_at DESC
             LIMIT ?2",
        )?;

        let items = stmt
            .query_map(params![search_pattern, limit as i64], |row| {
                Ok(HistoryItem {
                    id: row.get(0)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(1)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    mode_key: row.get(2)?,
                    audio_path: row.get(3)?,
                    transcript_raw: row.get(4)?,
                    output_final: row.get(5)?,
                    stt_provider: row.get(6)?,
                    stt_model: row.get(7)?,
                    llm_provider: row.get(8)?,
                    llm_model: row.get(9)?,
                    duration_ms: row.get::<_, i64>(10)? as u64,
                    error: row.get(11)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(items)
    }

    /// Clear all history
    pub fn clear_history(&self) -> Result<()> {
        self.conn.execute("DELETE FROM history_items", [])?;
        Ok(())
    }
}

/// Get the database path
pub fn get_database_path() -> Result<PathBuf> {
    let data_dir = directories::ProjectDirs::from("com", "whispertray", "WhisperTray")
        .ok_or_else(|| AppError::Config("Could not determine data directory".to_string()))?
        .data_dir()
        .to_path_buf();

    Ok(data_dir.join("history.db"))
}

/// Get the audio storage directory
pub fn get_audio_dir() -> Result<PathBuf> {
    let data_dir = directories::ProjectDirs::from("com", "whispertray", "WhisperTray")
        .ok_or_else(|| AppError::Config("Could not determine data directory".to_string()))?
        .data_dir()
        .to_path_buf();

    Ok(data_dir.join("audio"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_database_creation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.db");
        let db = Database::new(&path).unwrap();
        assert!(path.exists());
        drop(db);
    }

    #[test]
    fn test_insert_and_get_history() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.db");
        let db = Database::new(&path).unwrap();

        let item = HistoryItem {
            id: "test-id".to_string(),
            created_at: Utc::now(),
            mode_key: "voice_to_text".to_string(),
            audio_path: Some("/path/to/audio.wav".to_string()),
            transcript_raw: "Hello world".to_string(),
            output_final: "Hello world".to_string(),
            stt_provider: "whispercpp".to_string(),
            stt_model: "base.en".to_string(),
            llm_provider: None,
            llm_model: None,
            duration_ms: 1000,
            error: None,
        };

        db.insert_history(&item).unwrap();

        let retrieved = db.get_history_item("test-id").unwrap().unwrap();
        assert_eq!(retrieved.id, "test-id");
        assert_eq!(retrieved.transcript_raw, "Hello world");
    }

    #[test]
    fn test_get_history_pagination() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.db");
        let db = Database::new(&path).unwrap();

        // Insert 5 items
        for i in 0..5 {
            let item = HistoryItem {
                id: format!("test-id-{}", i),
                created_at: Utc::now(),
                mode_key: "voice_to_text".to_string(),
                audio_path: None,
                transcript_raw: format!("Item {}", i),
                output_final: format!("Item {}", i),
                stt_provider: "whispercpp".to_string(),
                stt_model: "base.en".to_string(),
                llm_provider: None,
                llm_model: None,
                duration_ms: 1000,
                error: None,
            };
            db.insert_history(&item).unwrap();
        }

        let items = db.get_history(2, 0).unwrap();
        assert_eq!(items.len(), 2);

        let items = db.get_history(10, 3).unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_delete_history() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.db");
        let db = Database::new(&path).unwrap();

        let item = HistoryItem {
            id: "test-id".to_string(),
            created_at: Utc::now(),
            mode_key: "voice_to_text".to_string(),
            audio_path: None,
            transcript_raw: "Hello".to_string(),
            output_final: "Hello".to_string(),
            stt_provider: "whispercpp".to_string(),
            stt_model: "base.en".to_string(),
            llm_provider: None,
            llm_model: None,
            duration_ms: 1000,
            error: None,
        };

        db.insert_history(&item).unwrap();
        assert!(db.get_history_item("test-id").unwrap().is_some());

        db.delete_history("test-id").unwrap();
        assert!(db.get_history_item("test-id").unwrap().is_none());
    }
}
