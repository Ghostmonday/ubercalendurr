pub struct Migrations;

impl Migrations {
    pub fn get_migrations() -> Vec<&'static str> {
        vec![
            // V1: Initial schema
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
                metadata TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_events_date ON events(date);
            CREATE INDEX IF NOT EXISTS idx_events_category ON events(category);
            CREATE INDEX IF NOT EXISTS idx_events_priority ON events(priority);
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            );
            "#,
        ]
    }
}
