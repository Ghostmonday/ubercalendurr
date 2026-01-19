use notify_rust::Notification;
use calendar_core::CalendarEvent;
use std::time::Duration;
use chrono::{Local, NaiveDateTime};

pub struct NotificationService {
    enabled: bool,
    play_sound: bool,
}

impl NotificationService {
    pub fn new(enabled: bool, play_sound: bool) -> Self {
        Self {
            enabled,
            play_sound,
        }
    }
    
    /// Check if an event should trigger a notification now
    pub fn should_notify(&self, event: &CalendarEvent, default_minutes: u32) -> bool {
        if !self.enabled {
            return false;
        }
        
        // Get reminder minutes (from event or default)
        let minutes_before = event.reminder
            .as_ref()
            .map(|r| r.minutes_before)
            .unwrap_or(default_minutes);
        
        // Parse event date/time
        let event_datetime = match self.parse_event_datetime(event) {
            Some(dt) => dt,
            None => return false, // Can't parse, skip
        };
        
        // Calculate notification time
        let notify_time = event_datetime - chrono::Duration::minutes(minutes_before as i64);
        let now = Local::now().naive_local();
        
        // Check if we're within 1 minute of notification time
        let diff = (notify_time - now).num_seconds().abs();
        diff < 60
    }
    
    /// Send OS notification for an event
    pub fn send_notification(&self, event: &CalendarEvent) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        
        let time_str = event.time.as_deref().unwrap_or("--:--");
        let body = format!(
            "{} at {}\n{}",
            event.date,
            time_str,
            event.notes.as_deref().unwrap_or("")
        );
        
        Notification::new()
            .summary(&event.event)
            .body(&body)
            .icon("calendar")
            .timeout(Duration::from_secs(10))
            .show()
            .map_err(|e| format!("Notification failed: {}", e))?;
        
        Ok(())
    }
    
    fn parse_event_datetime(&self, event: &CalendarEvent) -> Option<NaiveDateTime> {
        let date = chrono::NaiveDate::parse_from_str(&event.date, "%Y-%m-%d").ok()?;
        let time = event.time.as_ref()?;
        let time_parsed = chrono::NaiveTime::parse_from_str(time, "%H:%M").ok()?;
        Some(NaiveDateTime::new(date, time_parsed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use calendar_core::{CalendarEvent, ReminderConfig};
    
    #[test]
    fn test_should_notify_disabled() {
        let service = NotificationService::new(false, false);
        let event = CalendarEvent::new("Test".to_string(), "2026-01-20".to_string());
        
        assert!(!service.should_notify(&event, 15));
    }
    
    #[test]
    fn test_parse_event_datetime() {
        let service = NotificationService::new(true, true);
        let mut event = CalendarEvent::new("Test".to_string(), "2026-01-20".to_string());
        event.time = Some("14:00".to_string());
        
        let dt = service.parse_event_datetime(&event);
        assert!(dt.is_some());
    }
}
