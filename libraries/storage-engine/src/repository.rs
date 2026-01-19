use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use rusqlite::{Connection, OptionalExtension};
use crate::errors::{StorageError, Result};
use calendar_core::{CalendarEvent, Category, Priority, EventStatus, Visibility};

pub struct CalendarRepository {
    connection: Arc<Mutex<Connection>>,
    statement_cache: RwLock<StatementCache>,
}

struct StatementCache {
    get_by_id: Option<rusqlite::Statement<'static>>,
    get_by_date: Option<rusqlite::Statement<'static>>,
    get_by_date_range: Option<rusqlite::Statement<'static>>,
    search: Option<rusqlite::Statement<'static>>,
    insert: Option<rusqlite::Statement<'static>>,
    update: Option<rusqlite::Statement<'static>>,
    delete: Option<rusqlite::Statement<'static>>,
}

impl CalendarRepository {
    pub async fn new(db_path: &PathBuf) -> Result<Self> {
        let connection = tokio::task::spawn_blocking(|| {
            Connection::open(db_path)
        })
        .await
        .map_err(|e| StorageError::ConnectionFailed(e.to_string()))??;

        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.pragma_update(None, "synchronous", "NORMAL")?;
        connection.pragma_update(None, "cache_size", "-64000")?;

        // Initialize schema
        Self::init_schema(&connection)?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            statement_cache: RwLock::new(StatementCache {
                get_by_id: None,
                get_by_date: None,
                get_by_date_range: None,
                search: None,
                insert: None,
                update: None,
                delete: None,
            }),
        })
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                date TEXT NOT NULL,
                time TEXT,
                end_time TEXT,
                event TEXT NOT NULL,
                notes TEXT,
                priority TEXT NOT NULL DEFAULT 'medium',
                category TEXT NOT NULL DEFAULT 'other',
                color TEXT,
                tags TEXT,
                status TEXT NOT NULL DEFAULT 'confirmed',
                visibility TEXT NOT NULL DEFAULT 'private',
                metadata TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_date ON events(date)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_category ON events(category)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_priority ON events(priority)",
            [],
        )?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<CalendarEvent>> {
        let conn = self.connection.lock().await;
        
        let mut stmt = conn.prepare_cached(
            "SELECT * FROM events WHERE id = ?1"
        )?;

        stmt.query_row([], |row| {
            Self::row_to_event(row)
        })
        .optional()
        .map_err(|e| StorageError::QueryFailed(e.to_string()))
    }

    pub async fn get_by_date(&self, date: &str) -> Result<Vec<CalendarEvent>> {
        let conn = self.connection.lock().await;
        
        let mut stmt = conn.prepare_cached(
            "SELECT * FROM events WHERE date = ?1 ORDER BY time ASC"
        )?;

        let mut events = Vec::new();
        let mut rows = stmt.query([date])?;

        while let Ok(true) = rows.next() {
            events.push(Self::row_to_event(&rows)?);
        }

        Ok(events)
    }

    pub async fn get_by_date_range(
        &self, 
        start_date: &str, 
        end_date: &str
    ) -> Result<Vec<CalendarEvent>> {
        let conn = self.connection.lock().await;
        
        let mut stmt = conn.prepare_cached(
            "SELECT * FROM events 
             WHERE date >= ?1 AND date <= ?2 
             ORDER BY date ASC, time ASC"
        )?;

        let mut events = Vec::new();
        let mut rows = stmt.query([start_date, end_date])?;

        while let Ok(true) = rows.next() {
            events.push(Self::row_to_event(&rows)?);
        }

        Ok(events)
    }

    pub async fn save_event(&self, event: &CalendarEvent) -> Result<()> {
        let conn = self.connection.lock().await;
        
        conn.execute(
            r#"INSERT OR REPLACE INTO events (
                id, created_at, updated_at, date, time, end_time,
                event, notes, priority, category, color, tags,
                status, visibility, metadata
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)"#,
            &[
                &event.id.to_string(),
                &event.created_at.to_rfc3339(),
                &event.updated_at.to_rfc3339(),
                &event.date,
                event.time.as_ref(),
                event.end_time.as_ref(),
                &event.event,
                event.notes.as_ref(),
                &event.priority.as_str(),
                &event.category.as_str(),
                event.color.as_ref(),
                &serde_json::to_string(&event.tags).ok().as_ref(),
                &event.status.as_str(),
                &event.visibility.as_str(),
                &event.metadata.to_string(),
            ],
        )?;

        Ok(())
    }

    pub async fn delete_event(&self, id: &str) -> Result<bool> {
        let conn = self.connection.lock().await;
        
        let rows_affected = conn.execute(
            "DELETE FROM events WHERE id = ?1",
            [id],
        )?;

        Ok(rows_affected > 0)
    }

    pub async fn count(&self) -> Result<u64> {
        let conn = self.connection.lock().await;
        
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM events",
            [],
            |row| row.get(0),
        )?;

        Ok(count as u64)
    }

    fn row_to_event(row: &rusqlite::Row) -> Result<CalendarEvent, rusqlite::Error> {
        let id: String = row.get(0)?;
        let created_at: String = row.get(1)?;
        let updated_at: String = row.get(2)?;
        let date: String = row.get(3)?;
        let time: Option<String> = row.get(4)?;
        let end_time: Option<String> = row.get(5)?;
        let event: String = row.get(6)?;
        let notes: Option<String> = row.get(7)?;
        let priority: String = row.get(8)?;
        let category: String = row.get(9)?;
        let color: Option<String> = row.get(10)?;
        let tags_str: Option<String> = row.get(11)?;
        let status: String = row.get(12)?;
        let visibility: String = row.get(13)?;
        let metadata_str: Option<String> = row.get(14)?;

        let tags: Vec<String> = tags_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();

        let metadata: serde_json::Value = metadata_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();

        Ok(CalendarEvent {
            id: id.parse().unwrap_or_else(|_| uuid::Uuid::new_v4()),
            created_at: created_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: updated_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
            date,
            time,
            end_time,
            event,
            notes,
            priority: priority.parse().unwrap_or(Priority::Medium),
            category: category.parse().unwrap_or(Category::Other),
            color,
            tags,
            status: status.parse().unwrap_or(EventStatus::Confirmed),
            visibility: visibility.parse().unwrap_or(Visibility::Private),
            recurring: None,
            reminder: None,
            location: None,
            metadata,
        })
    }
}
