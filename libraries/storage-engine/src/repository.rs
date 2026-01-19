use std::path::PathBuf;
use rusqlite::{Connection, OptionalExtension};
use calendar_core::{AppError, AppResult};
use calendar_core::{CalendarEvent, Category, Priority, EventStatus, Visibility};

pub struct CalendarRepository {
    connection: Connection,
}

impl CalendarRepository {
    pub fn new(db_path: &PathBuf) -> AppResult<Self> {
        let connection = Connection::open(db_path)
            .map_err(|e| AppError::Database(format!("Connection failed: {}", e)))?;

        connection.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| AppError::Database(format!("Failed to set WAL mode: {}", e)))?;
        connection.pragma_update(None, "synchronous", "NORMAL")
            .map_err(|e| AppError::Database(format!("Failed to set synchronous: {}", e)))?;
        connection.pragma_update(None, "cache_size", "-64000")
            .map_err(|e| AppError::Database(format!("Failed to set cache size: {}", e)))?;

        // Initialize schema
        Self::init_schema(&connection)?;

        Ok(Self {
            connection,
        })
    }

    fn init_schema(conn: &Connection) -> AppResult<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS events (
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
                recurring TEXT,
                reminder TEXT,
                location TEXT,
                metadata TEXT NOT NULL DEFAULT '{}'
            );
            
            CREATE INDEX IF NOT EXISTS idx_events_date ON events(date);
            CREATE INDEX IF NOT EXISTS idx_events_category ON events(category);
            CREATE INDEX IF NOT EXISTS idx_events_priority ON events(priority);
            
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            
            INSERT OR IGNORE INTO schema_migrations (version) VALUES (1);
            "#
        )?;

        Ok(())
    }

    pub fn get_by_id(&self, id: &str) -> AppResult<Option<CalendarEvent>> {
        let mut stmt = self.connection.prepare(
            "SELECT * FROM events WHERE id = ?1"
        )
        .map_err(|e| AppError::Database(format!("Prepare failed: {}", e)))?;

        stmt.query_row([id], |row| {
            Self::row_to_event(row)
        })
        .optional()
        .map_err(|e| AppError::Database(format!("Query failed: {}", e)))
    }

    pub fn get_by_date(&self, date: &str) -> AppResult<Vec<CalendarEvent>> {
        let mut stmt = self.connection.prepare(
            "SELECT * FROM events WHERE date = ?1 ORDER BY time ASC"
        )
        .map_err(|e| AppError::Database(format!("Prepare failed: {}", e)))?;

        let mut events = Vec::new();
        let mut rows = stmt.query([date])
            .map_err(|e| AppError::Database(format!("Query failed: {}", e)))?;

        while let Ok(true) = rows.next() {
            events.push(Self::row_to_event(&rows)?);
        }

        Ok(events)
    }

    pub fn get_by_date_range(
        &self, 
        start_date: &str, 
        end_date: &str
    ) -> AppResult<Vec<CalendarEvent>> {
        // First, get all events (including recurring ones)
        let mut stmt = self.connection.prepare(
            "SELECT * FROM events 
             WHERE (date >= ?1 AND date <= ?2) 
             OR recurring IS NOT NULL
             ORDER BY date ASC, time ASC"
        )
        .map_err(|e| AppError::Database(format!("Prepare failed: {}", e)))?;

        let mut base_events = Vec::new();
        let mut rows = stmt.query([start_date, end_date])
            .map_err(|e| AppError::Database(format!("Query failed: {}", e)))?;

        while let Ok(true) = rows.next() {
            base_events.push(Self::row_to_event(&rows)?);
        }
        
        // Expand recurring events into instances
        let mut all_events = Vec::new();
        
        for event in base_events {
            if let Some(ref recurring) = event.recurring {
                // Generate occurrences for this recurring event
                let occurrences = recurring.generate_occurrences(&event.date, Some(365));
                
                // Filter occurrences within the requested range
                for occurrence_date in occurrences {
                    if occurrence_date >= start_date && occurrence_date <= end_date {
                        // Create instance of recurring event for this date
                        let mut instance = event.clone();
                        instance.date = occurrence_date;
                        instance.id = uuid::Uuid::new_v4(); // New ID for each instance
                        all_events.push(instance);
                    }
                }
            } else {
                // Non-recurring event, add as-is (if in range)
                if event.date >= start_date && event.date <= end_date {
                    all_events.push(event);
                }
            }
        }
        
        // Sort by date and time
        all_events.sort_by(|a, b| {
            match a.date.cmp(&b.date) {
                std::cmp::Ordering::Equal => {
                    a.time.as_ref().unwrap_or(&"00:00".to_string())
                        .cmp(b.time.as_ref().unwrap_or(&"00:00".to_string()))
                }
                other => other,
            }
        });

        Ok(all_events)
    }

    pub fn save_event(&self, event: &CalendarEvent) -> AppResult<()> {
        let recurring_json = event.recurring.as_ref()
            .and_then(|r| serde_json::to_string(r).ok());
        let reminder_json = event.reminder.as_ref()
            .and_then(|r| serde_json::to_string(r).ok());
        let location_json = event.location.as_ref()
            .and_then(|l| serde_json::to_string(l).ok());
        let metadata_json = serde_json::to_string(&event.metadata)
            .unwrap_or_else(|_| "{}".to_string());
        
        self.connection.execute(
            r#"INSERT OR REPLACE INTO events (
                id, created_at, updated_at, date, time, end_time,
                event, notes, priority, category, color, tags,
                status, visibility, recurring, reminder, location, metadata
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)"#,
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
                recurring_json.as_ref(),
                reminder_json.as_ref(),
                location_json.as_ref(),
                &metadata_json,
            ],
        )
        .map_err(|e| AppError::Database(format!("Save failed: {}", e)))?;

        Ok(())
    }

    pub fn delete_event(&self, id: &str) -> AppResult<bool> {
        let rows_affected = self.connection.execute(
            "DELETE FROM events WHERE id = ?1",
            [id],
        )
        .map_err(|e| AppError::Database(format!("Delete failed: {}", e)))?;

        Ok(rows_affected > 0)
    }

    pub fn count(&self) -> AppResult<u64> {
        let count: i64 = self.connection.query_row(
            "SELECT COUNT(*) FROM events",
            [],
            |row| row.get(0),
        )
        .map_err(|e| AppError::Database(format!("Count failed: {}", e)))?;

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
        let recurring_str: Option<String> = row.get(14)?;
        let reminder_str: Option<String> = row.get(15)?;
        let location_str: Option<String> = row.get(16)?;
        let metadata_str: Option<String> = row.get(17)?;

        let tags: Vec<String> = tags_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();

        let recurring: Option<calendar_core::RecurrenceConfig> = recurring_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok());

        let reminder: Option<calendar_core::ReminderConfig> = reminder_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok());

        let location: Option<calendar_core::Location> = location_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok());

        let metadata: serde_json::Value = metadata_str
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_else(|| serde_json::json!({}));

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
            recurring,
            reminder,
            location,
            metadata,
        })
    }

    /// Check for conflicting events (overlapping time on same date)
    pub fn check_conflicts(&self, event: &CalendarEvent) -> AppResult<Vec<String>> {
        if event.time.is_none() || event.end_time.is_none() {
            return Ok(Vec::new()); // All-day or no-time events don't conflict
        }

        let mut conflicts = Vec::new();
        
        let mut stmt = self.connection.prepare(
            "SELECT id FROM events 
             WHERE date = ?1 
             AND id != ?2
             AND time IS NOT NULL 
             AND end_time IS NOT NULL"
        )
        .map_err(|e| AppError::Database(format!("Prepare failed: {}", e)))?;

        let rows = stmt.query_map(
            [&event.date, &event.id.to_string()],
            |row| {
                let other_id: String = row.get(0)?;
                Ok(other_id)
            }
        )
        .map_err(|e| AppError::Database(format!("Query failed: {}", e)))?;

        for row_result in rows {
            let other_id = row_result
                .map_err(|e| AppError::Database(format!("Row read failed: {}", e)))?;
            
            // Get the other event to check time overlap
            if let Ok(Some(other_event)) = self.get_by_id(&other_id) {
                if let (Some(start1), Some(end1), Some(start2), Some(end2)) = 
                    (&event.time, &event.end_time, &other_event.time, &other_event.end_time) {
                    // Check if times overlap
                    if (start1 < end2 && end1 > start2) || (start2 < end1 && end2 > start1) {
                        conflicts.push(other_id);
                    }
                }
            }
        }

        Ok(conflicts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use calendar_core::{Priority, Category, RecurrenceConfig, RecurrenceFrequency, ReminderConfig, Location, LocationType};
    
    fn create_test_repo() -> CalendarRepository {
        let db_path = PathBuf::from(":memory:");
        CalendarRepository::new(&db_path).unwrap()
    }
    
    fn create_test_event(title: &str, date: &str) -> CalendarEvent {
        let mut event = CalendarEvent::new(title.to_string(), date.to_string());
        event.time = Some("14:00".to_string());
        event.end_time = Some("15:00".to_string());
        event
    }
    
    #[test]
    fn test_crud_lifecycle() {
        let repo = create_test_repo();
        let event = create_test_event("Test", "2026-01-20");
        
        // Create
        repo.save_event(&event).unwrap();
        
        // Read
        let retrieved = repo.get_by_id(&event.id.to_string()).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.as_ref().unwrap().event, "Test");
        
        // Update
        let mut updated = retrieved.unwrap();
        updated.event = "Updated Test".to_string();
        repo.save_event(&updated).unwrap();
        
        let retrieved2 = repo.get_by_id(&updated.id.to_string()).unwrap();
        assert_eq!(retrieved2.unwrap().event, "Updated Test");
        
        // Delete
        let deleted = repo.delete_event(&updated.id.to_string()).unwrap();
        assert!(deleted);
        
        let retrieved3 = repo.get_by_id(&updated.id.to_string()).unwrap();
        assert!(retrieved3.is_none());
    }
    
    #[test]
    fn test_date_range_query() {
        let repo = create_test_repo();
        
        repo.save_event(&create_test_event("Event 1", "2026-01-20")).unwrap();
        repo.save_event(&create_test_event("Event 2", "2026-01-21")).unwrap();
        repo.save_event(&create_test_event("Event 3", "2026-01-25")).unwrap();
        
        let events = repo.get_by_date_range("2026-01-20", "2026-01-22").unwrap();
        assert_eq!(events.len(), 2);
    }
    
    #[test]
    fn test_recurring_json_roundtrip() {
        let repo = create_test_repo();
        let mut event = create_test_event("Recurring", "2026-01-20");
        
        event.recurring = Some(RecurrenceConfig {
            frequency: RecurrenceFrequency::Weekly,
            interval: 1,
            days_of_week: vec![1, 3, 5],
            end_date: Some("2026-12-31".to_string()),
            occurrences: None,
            except_dates: vec!["2026-02-10".to_string()],
        });
        
        repo.save_event(&event).unwrap();
        let retrieved = repo.get_by_id(&event.id.to_string()).unwrap().unwrap();
        
        assert!(retrieved.recurring.is_some());
        let rec = retrieved.recurring.unwrap();
        assert_eq!(rec.frequency, RecurrenceFrequency::Weekly);
        assert_eq!(rec.days_of_week, vec![1, 3, 5]);
        assert_eq!(rec.except_dates, vec!["2026-02-10".to_string()]);
    }
    
    #[test]
    fn test_reminder_json_roundtrip() {
        let repo = create_test_repo();
        let mut event = create_test_event("With Reminder", "2026-01-20");
        
        event.reminder = Some(ReminderConfig {
            minutes_before: 30,
            repeat_minutes: Some(10),
            max_reminders: 5,
        });
        
        repo.save_event(&event).unwrap();
        let retrieved = repo.get_by_id(&event.id.to_string()).unwrap().unwrap();
        
        assert!(retrieved.reminder.is_some());
        let rem = retrieved.reminder.unwrap();
        assert_eq!(rem.minutes_before, 30);
        assert_eq!(rem.repeat_minutes, Some(10));
    }
    
    #[test]
    fn test_location_json_roundtrip() {
        let repo = create_test_repo();
        let mut event = create_test_event("With Location", "2026-01-20");
        
        event.location = Some(Location {
            location_type: LocationType::Physical,
            address: "123 Main St".to_string(),
            coordinates: None,
        });
        
        repo.save_event(&event).unwrap();
        let retrieved = repo.get_by_id(&event.id.to_string()).unwrap().unwrap();
        
        assert!(retrieved.location.is_some());
        assert_eq!(retrieved.location.unwrap().address, "123 Main St");
    }
    
    #[test]
    fn test_conflict_detection_overlapping() {
        let repo = create_test_repo();
        
        let mut event1 = create_test_event("Event 1", "2026-01-20");
        event1.time = Some("14:00".to_string());
        event1.end_time = Some("15:00".to_string());
        repo.save_event(&event1).unwrap();
        
        let mut event2 = create_test_event("Event 2", "2026-01-20");
        event2.time = Some("14:30".to_string());
        event2.end_time = Some("15:30".to_string());
        
        let conflicts = repo.check_conflicts(&event2).unwrap();
        assert!(conflicts.contains(&event1.id.to_string()));
    }
    
    #[test]
    fn test_conflict_detection_no_overlap() {
        let repo = create_test_repo();
        
        let mut event1 = create_test_event("Event 1", "2026-01-20");
        event1.time = Some("14:00".to_string());
        event1.end_time = Some("15:00".to_string());
        repo.save_event(&event1).unwrap();
        
        let mut event2 = create_test_event("Event 2", "2026-01-20");
        event2.time = Some("15:00".to_string());
        event2.end_time = Some("16:00".to_string());
        
        let conflicts = repo.check_conflicts(&event2).unwrap();
        assert_eq!(conflicts.len(), 0);
    }
    
    #[test]
    fn test_count() {
        let repo = create_test_repo();
        
        assert_eq!(repo.count().unwrap(), 0);
        
        repo.save_event(&create_test_event("Event 1", "2026-01-20")).unwrap();
        repo.save_event(&create_test_event("Event 2", "2026-01-21")).unwrap();
        
        assert_eq!(repo.count().unwrap(), 2);
    }
    
    #[test]
    fn test_recurring_events_in_date_range() {
        let repo = create_test_repo();
        
        let mut recurring_event = create_test_event("Weekly Meeting", "2026-01-20");
        recurring_event.recurring = Some(RecurrenceConfig {
            frequency: RecurrenceFrequency::Weekly,
            interval: 1,
            days_of_week: vec![],
            end_date: None,
            occurrences: Some(4),
            except_dates: vec![],
        });
        
        repo.save_event(&recurring_event).unwrap();
        
        // Query should return 4 instances
        let events = repo.get_by_date_range("2026-01-01", "2026-02-28").unwrap();
        
        let meeting_events: Vec<_> = events.iter()
            .filter(|e| e.event == "Weekly Meeting")
            .collect();
        
        assert_eq!(meeting_events.len(), 4);
        
        // Verify dates are 7 days apart
        assert_eq!(meeting_events[0].date, "2026-01-20");
        assert_eq!(meeting_events[1].date, "2026-01-27");
    }
}
