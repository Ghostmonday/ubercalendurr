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

    pub fn validate(&self) -> crate::AppResult<()> {
        if self.event.trim().is_empty() {
            return Err(crate::AppError::Validation("Event title cannot be empty".to_string()));
        }
        Ok(())
    }

    pub fn effective_color(&self) -> &str {
        self.color.as_deref().unwrap_or_else(|| self.category.color())
    }

    /// Create CalendarEvent from parsed event data (from SimpleParser or AI)
    pub fn from_parsed(
        event: String,
        date: String,
        time: Option<String>,
        end_time: Option<String>,
        notes: Option<String>,
        priority: String,
        category: String,
        tags: Vec<String>,
        metadata: serde_json::Value,
        recurring: Option<RecurrenceConfig>,
    ) -> Self {
        let now = chrono::Utc::now();
        
        // Apply inferred defaults
        let priority_enum = priority.parse().unwrap_or(Priority::Medium);
        let category_enum = category.parse().unwrap_or(Category::Other);
        
        // Ensure metadata has source
        let mut final_metadata = metadata;
        if !final_metadata.is_object() {
            final_metadata = serde_json::json!({});
        }
        if !final_metadata.get("source").is_some() {
            final_metadata["source"] = serde_json::json!("Parsed");
        }
        
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            date,
            time,
            end_time,
            event,
            notes,
            priority: priority_enum,
            category: category_enum,
            color: None,
            tags,
            status: EventStatus::Confirmed,
            visibility: Visibility::Private,
            recurring,
            reminder: None,
            location: None,
            metadata: final_metadata,
        }
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

impl RecurrenceConfig {
    /// Generate occurrence dates for this recurrence pattern
    pub fn generate_occurrences(
        &self,
        start_date: &str,
        limit: Option<u32>,
    ) -> Vec<String> {
        if self.frequency == RecurrenceFrequency::None {
            return vec![start_date.to_string()];
        }

        let mut occurrences = Vec::new();
        let Some(mut current) = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").ok() else {
            return vec![start_date.to_string()];
        };
        
        let max_occurrences = limit.or(self.occurrences).unwrap_or(365);
        let mut count = 0;

        while count < max_occurrences {
            // Check if we've hit end_date
            if let Some(ref end_date_str) = self.end_date {
                if let Ok(end_date) = chrono::NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d") {
                    if current > end_date {
                        break;
                    }
                }
            }

            // Check if this date is excluded
            let date_str = current.format("%Y-%m-%d").to_string();
            if !self.except_dates.contains(&date_str) {
                occurrences.push(date_str.clone());
                count += 1;
            }

            // Advance to next occurrence
            current = match self.frequency {
                RecurrenceFrequency::Daily => {
                    current.checked_add_signed(chrono::Duration::days(self.interval as i64))?
                }
                RecurrenceFrequency::Weekly => {
                    if self.days_of_week.is_empty() {
                        match current.checked_add_signed(chrono::Duration::weeks(self.interval as i64)) {
                            Some(d) => d,
                            None => break,
                        }
                    } else {
                        // Find next matching weekday
                        let mut next = current;
                        for _ in 0..(7 * self.interval as i64) {
                            next = match next.succ_opt() {
                                Some(d) => d,
                                None => break,
                            };
                            let weekday_num = next.weekday().num_days_from_sunday() as u8;
                            if self.days_of_week.contains(&weekday_num) {
                                break;
                            }
                        }
                        next
                    }
                }
                RecurrenceFrequency::Biweekly => {
                    match current.checked_add_signed(chrono::Duration::weeks(2 * self.interval as i64)) {
                        Some(d) => d,
                        None => break,
                    }
                }
                RecurrenceFrequency::Monthly => {
                    // Add one month
                    let mut year = current.year();
                    let mut month = current.month();
                    month += self.interval;
                    while month > 12 {
                        month -= 12;
                        year += 1;
                    }
                    chrono::NaiveDate::from_ymd_opt(year, month, current.day())
                        .unwrap_or_else(|| current + chrono::Duration::days(30 * self.interval as i64))
                }
                RecurrenceFrequency::Yearly => {
                    chrono::NaiveDate::from_ymd_opt(
                        current.year() + self.interval as i32,
                        current.month(),
                        current.day()
                    ).unwrap_or_else(|| current + chrono::Duration::days(365 * self.interval as i64))
                }
                _ => break,
            };
        }

        occurrences
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_empty_title() {
        let event = CalendarEvent::new("".to_string(), "2026-01-20".to_string());
        assert!(event.validate().is_err());
    }
    
    #[test]
    fn test_validate_valid_event() {
        let event = CalendarEvent::new("Meeting".to_string(), "2026-01-20".to_string());
        assert!(event.validate().is_ok());
    }
    
    #[test]
    fn test_from_parsed_with_metadata() {
        let event = CalendarEvent::from_parsed(
            "Team meeting".to_string(),
            "2026-01-20".to_string(),
            Some("14:00".to_string()),
            Some("15:00".to_string()),
            None,
            "high".to_string(),
            "work".to_string(),
            vec!["meeting".to_string()],
            serde_json::json!({"project": "DemoProject"}),
            None,
        );
        
        assert_eq!(event.event, "Team meeting");
        assert_eq!(event.priority, Priority::High);
        assert_eq!(event.category, Category::Work);
        assert_eq!(event.metadata["project"], "DemoProject");
    }
    
    #[test]
    fn test_from_parsed_with_recurring() {
        let recurring = RecurrenceConfig {
            frequency: RecurrenceFrequency::Daily,
            interval: 1,
            days_of_week: vec![],
            end_date: None,
            occurrences: Some(5),
            except_dates: vec![],
        };
        
        let event = CalendarEvent::from_parsed(
            "Daily standup".to_string(),
            "2026-01-20".to_string(),
            Some("09:00".to_string()),
            Some("09:30".to_string()),
            None,
            "medium".to_string(),
            "work".to_string(),
            vec!["standup".to_string()],
            serde_json::json!({}),
            Some(recurring),
        );
        
        assert!(event.recurring.is_some());
        assert_eq!(event.recurring.unwrap().frequency, RecurrenceFrequency::Daily);
    }
    
    #[test]
    fn test_generate_occurrences_daily() {
        let config = RecurrenceConfig {
            frequency: RecurrenceFrequency::Daily,
            interval: 1,
            days_of_week: vec![],
            end_date: None,
            occurrences: Some(5),
            except_dates: vec![],
        };
        
        let occurrences = config.generate_occurrences("2026-01-20", Some(5));
        assert_eq!(occurrences.len(), 5);
        assert_eq!(occurrences[0], "2026-01-20");
        assert_eq!(occurrences[1], "2026-01-21");
    }
    
    #[test]
    fn test_generate_occurrences_weekly() {
        let config = RecurrenceConfig {
            frequency: RecurrenceFrequency::Weekly,
            interval: 1,
            days_of_week: vec![],
            end_date: None,
            occurrences: Some(3),
            except_dates: vec![],
        };
        
        let occurrences = config.generate_occurrences("2026-01-20", Some(3));
        assert_eq!(occurrences.len(), 3);
    }
    
    #[test]
    fn test_priority_parsing() {
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("urgent".parse::<Priority>().unwrap(), Priority::Urgent);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
    }
    
    #[test]
    fn test_category_parsing() {
        assert_eq!("work".parse::<Category>().unwrap(), Category::Work);
        assert_eq!("personal".parse::<Category>().unwrap(), Category::Personal);
        assert_eq!("health".parse::<Category>().unwrap(), Category::Health);
        assert_eq!("social".parse::<Category>().unwrap(), Category::Social);
    }
}
