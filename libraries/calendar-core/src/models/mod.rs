use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashSet;

pub mod prelude {
    pub use super::{CalendarEvent, Priority, Category, EventStatus, Visibility};
    pub use super::{RecurrenceConfig, RecurrenceFrequency};
    pub use super::{ReminderConfig, Location, LocationType};
    pub use super::Coordinates;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub date: String,
    pub time: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub event: String,
    pub notes: Option<String>,
    pub priority: Priority,
    pub recurring: Option<RecurrenceConfig>,
    pub reminder: Option<ReminderConfig>,
    pub location: Option<Location>,
    pub category: Category,
    pub color: Option<String>,
    pub tags: Vec<String>,
    pub status: EventStatus,
    pub visibility: Visibility,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

impl CalendarEvent {
    pub fn new(event: String, date: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            date,
            time: None,
            end_time: None,
            event,
            notes: None,
            priority: Priority::Medium,
            recurring: None,
            reminder: None,
            location: None,
            category: Category::Other,
            color: None,
            tags: Vec::new(),
            status: EventStatus::Confirmed,
            visibility: Visibility::Private,
            metadata: serde_json::json!({}),
        }
    }

    pub fn validate(&self) -> Result<(), crate::CoreError> {
        if self.event.trim().is_empty() {
            return Err(crate::CoreError::ValidationError("Event title cannot be empty".to_string()));
        }
        Ok(())
    }

    pub fn effective_color(&self) -> &str {
        self.color.as_deref().unwrap_or_else(|| self.category.color())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl Priority {
    pub fn level(&self) -> u8 {
        match self {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
            Priority::Urgent => 3,
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🟠",
            Priority::Urgent => "🔴",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Urgent => "urgent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Work,
    Personal,
    Health,
    Social,
    Finance,
    Education,
    Other,
}

impl Default for Category {
    fn default() -> Self {
        Category::Other
    }
}

impl Category {
    pub fn color(&self) -> &'static str {
        match self {
            Category::Work => "#3B82F6",
            Category::Personal => "#10B981",
            Category::Health => "#EF4444",
            Category::Social => "#8B5CF6",
            Category::Finance => "#F59E0B",
            Category::Education => "#6366F1",
            Category::Other => "#6B7280",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Category::Work => "Work",
            Category::Personal => "Personal",
            Category::Health => "Health",
            Category::Social => "Social",
            Category::Finance => "Finance",
            Category::Education => "Education",
            Category::Other => "Other",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Category::Work => "work",
            Category::Personal => "personal",
            Category::Health => "health",
            Category::Social => "social",
            Category::Finance => "finance",
            Category::Education => "education",
            Category::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Tentative,
    Confirmed,
    Cancelled,
    Completed,
}

impl Default for EventStatus {
    fn default() -> Self {
        EventStatus::Confirmed
    }
}

impl EventStatus {
    pub fn as_str(&self) -> &str {
        match self {
            EventStatus::Tentative => "tentative",
            EventStatus::Confirmed => "confirmed",
            EventStatus::Cancelled => "cancelled",
            EventStatus::Completed => "completed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}

impl Visibility {
    pub fn as_str(&self) -> &str {
        match self {
            Visibility::Public => "public",
            Visibility::Private => "private",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceConfig {
    pub frequency: RecurrenceFrequency,
    #[serde(default = "default_interval")]
    pub interval: u32,
    #[serde(default)]
    pub days_of_week: Vec<u8>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub occurrences: Option<u32>,
    #[serde(rename = "exceptDates", default)]
    pub except_dates: Vec<String>,
}

impl Default for RecurrenceConfig {
    fn default() -> Self {
        Self {
            frequency: RecurrenceFrequency::None,
            interval: 1,
            days_of_week: Vec::new(),
            end_date: None,
            occurrences: None,
            except_dates: Vec::new(),
        }
    }
}

fn default_interval() -> u32 {
    1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecurrenceFrequency {
    None,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Yearly,
    Custom,
}

impl RecurrenceFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            RecurrenceFrequency::None => "none",
            RecurrenceFrequency::Daily => "daily",
            RecurrenceFrequency::Weekly => "weekly",
            RecurrenceFrequency::Biweekly => "biweekly",
            RecurrenceFrequency::Monthly => "monthly",
            RecurrenceFrequency::Yearly => "yearly",
            RecurrenceFrequency::Custom => "custom",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderConfig {
    #[serde(rename = "minutesBefore")]
    pub minutes_before: u32,
    #[serde(rename = "repeatMinutes")]
    pub repeat_minutes: Option<u32>,
    #[serde(rename = "maxReminders")]
    pub max_reminders: u32,
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self {
            minutes_before: 15,
            repeat_minutes: None,
            max_reminders: 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub location_type: LocationType,
    pub address: String,
    pub coordinates: Option<Coordinates>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    Physical,
    Virtual,
}

impl LocationType {
    pub fn as_str(&self) -> &str {
        match self {
            LocationType::Physical => "physical",
            LocationType::Virtual => "virtual",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}
