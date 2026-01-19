use std::path::PathBuf;
use storage_engine::{CalendarRepository, Result};

pub use storage_engine::CalendarRepository;

pub struct Repository(CalendarRepository);

impl Repository {
    pub async fn new(db_path: &PathBuf) -> Result<Self> {
        Ok(Self(CalendarRepository::new(db_path).await?))
    }

    pub async fn get_today_events(&self) -> Result<Vec<calendar_core::CalendarEvent>> {
        let today = chrono::Local::now()
            .format("%Y-%m-%d")
            .to_string();
        self.0.get_by_date(&today).await
    }
}
