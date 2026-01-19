use std::path::PathBuf;
use storage_engine::CalendarRepository;
use calendar_core::AppResult;

pub use storage_engine::CalendarRepository;

pub struct Repository(pub CalendarRepository);

impl Repository {
    pub fn new(db_path: &PathBuf) -> AppResult<Self> {
        Ok(Self(CalendarRepository::new(db_path)?))
    }

    pub fn get_today_events(&self) -> AppResult<Vec<calendar_core::CalendarEvent>> {
        let today = chrono::Local::now()
            .format("%Y-%m-%d")
            .to_string();
        self.0.get_by_date(&today)
    }

    pub fn save_event(&self, event: &calendar_core::CalendarEvent) -> AppResult<()> {
        self.0.save_event(event)
    }
}
