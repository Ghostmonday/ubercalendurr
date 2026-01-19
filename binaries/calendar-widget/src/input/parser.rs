use regex::Regex;
use chrono::{Local, Duration, NaiveTime};
use once_cell::sync::Lazy;

static TIME_REGEX: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"(?i)(\d{1,2})(?::(\d{2}))?\s*(am|pm)?").unwrap());

static DATE_REGEX: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"(?i)(today|tomorrow|next\s+(monday|tuesday|wednesday|thursday|friday|saturday|sunday))").unwrap());

static RELATIVE_TIME_REGEX: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"(?i)(morning|afternoon|evening|noon|lunch|dinner)").unwrap());

static RECURRING_REGEX: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"(?i)(every|daily|weekly|monthly|yearly)(\s+(monday|tuesday|wednesday|thursday|friday|saturday|sunday))?").unwrap());

#[derive(Debug, Clone)]
pub struct ParsedEvent {
    pub event: String,
    pub date: String,
    pub time: Option<String>,
    pub end_time: Option<String>,
    pub notes: Option<String>,
    pub priority: String,
    pub category: String,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub recurring: Option<calendar_core::RecurrenceConfig>,
}

pub struct SimpleParser;

impl SimpleParser {
    pub fn parse(&self, input: &str) -> Result<ParsedEvent, String> {
        let input_lower = input.to_lowercase();
        
        let mut event = ParsedEvent {
            event: Self::extract_event_title(input),
            date: Self::parse_date(input).unwrap_or_else(|| Self::today()),
            time: Self::parse_time(input),
            end_time: None,
            notes: None,
            priority: Self::detect_priority(input),
            category: Self::detect_category(input),
            tags: Self::extract_tags(input),
            metadata: Self::extract_project_metadata(input),
            recurring: Self::detect_recurring(input),
        };

        // Auto-infer end_time if time is set (default 1 hour duration)
        if event.time.is_some() && event.end_time.is_none() {
            if let Some(time_str) = &event.time {
                if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
                    let end_time = time + Duration::hours(1);
                    event.end_time = Some(end_time.format("%H:%M").to_string());
                }
            }
        }

        Ok(event)
    }

    fn extract_event_title(input: &str) -> String {
        let mut cleaned = input.to_string();
        
        // Remove common date/time words
        let patterns = vec![
            "today", "tomorrow", "next", "monday", "tuesday", "wednesday",
            "thursday", "friday", "saturday", "sunday",
            "am", "pm", "morning", "afternoon", "evening", "noon", "lunch", "dinner",
            "at", "on", "in"
        ];
        
        for pattern in patterns {
            cleaned = cleaned.replace(pattern, "");
        }
        
        // Remove time patterns
        cleaned = TIME_REGEX.replace_all(&cleaned, "").to_string();
        
        // Remove extra whitespace
        cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn parse_date(input: &str) -> Option<String> {
        let input_lower = input.to_lowercase();
        let today = Local::now().date_naive();
        
        if input_lower.contains("today") {
            Some(today.format("%Y-%m-%d").to_string())
        } else if input_lower.contains("tomorrow") {
            Some((today + Duration::days(1)).format("%Y-%m-%d").to_string())
        } else if let Some(caps) = DATE_REGEX.captures(input) {
            if let Some(weekday_str) = caps.get(2) {
                let weekday = Self::parse_weekday(weekday_str.as_str())?;
                let mut date = today;
                while date.weekday() != weekday {
                    date = date.succ_opt()?;
                }
                Some(date.format("%Y-%m-%d").to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_weekday(input: &str) -> Option<chrono::Weekday> {
        match input.to_lowercase().as_str() {
            "monday" => Some(chrono::Weekday::Mon),
            "tuesday" => Some(chrono::Weekday::Tue),
            "wednesday" => Some(chrono::Weekday::Wed),
            "thursday" => Some(chrono::Weekday::Thu),
            "friday" => Some(chrono::Weekday::Fri),
            "saturday" => Some(chrono::Weekday::Sat),
            "sunday" => Some(chrono::Weekday::Sun),
            _ => None,
        }
    }

    fn parse_time(input: &str) -> Option<String> {
        // Try explicit time first (e.g., "2pm", "14:00")
        if let Some(caps) = TIME_REGEX.captures(input) {
            let hour: u32 = caps.get(1)?.as_str().parse().ok()?;
            let minute: u32 = caps.get(2).map(|m| m.as_str().parse().ok()).flatten().unwrap_or(0);
            let am_pm = caps.get(3).map(|m| m.as_str().to_lowercase());
            
            let hour_24 = match am_pm.as_deref() {
                Some("pm") if hour != 12 => hour + 12,
                Some("am") if hour == 12 => 0,
                _ => hour,
            };
            
            return Some(format!("{:02}:{:02}", hour_24, minute));
        }
        
        // Try relative time words
        if let Some(caps) = RELATIVE_TIME_REGEX.captures(input) {
            let time_word = caps.get(1)?.as_str().to_lowercase();
            return match time_word.as_str() {
                "morning" => Some("09:00".to_string()),
                "noon" | "lunch" => Some("12:00".to_string()),
                "afternoon" => Some("14:00".to_string()),
                "evening" | "dinner" => Some("18:00".to_string()),
                _ => None,
            };
        }
        
        None
    }

    fn today() -> String {
        Local::now().date_naive().format("%Y-%m-%d").to_string()
    }

    fn detect_priority(input: &str) -> String {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("urgent") || input_lower.contains("critical") || 
           input_lower.contains("asap") || input_lower.contains("deadline") {
            "urgent".to_string()
        } else if input_lower.contains("important") || input_lower.contains("priority") {
            "high".to_string()
        } else if input_lower.contains("maybe") || input_lower.contains("tentative") ||
                  input_lower.contains("optional") {
            "low".to_string()
        } else {
            "medium".to_string()
        }
    }

    fn detect_category(input: &str) -> String {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("lunch") || input_lower.contains("dinner") || 
                  input_lower.contains("coffee") || input_lower.contains("meet") {
            "social".to_string()
        } else if input_lower.contains("doctor") || input_lower.contains("dentist") || 
                  input_lower.contains("health") || input_lower.contains("appointment") {
            "health".to_string()
        } else if input_lower.contains("meeting") || input_lower.contains("work") || 
                  input_lower.contains("call") || input_lower.contains("sync") {
            "work".to_string()
        } else {
            "personal".to_string()
        }
    }

    fn extract_project_metadata(input: &str) -> serde_json::Value {
        let mut metadata = serde_json::json!({});
        let input_lower = input.to_lowercase();
        
        metadata["source"] = serde_json::Value::String("SimpleParser".to_string());
        metadata
    }

    fn extract_tags(input: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let input_lower = input.to_lowercase();
        
        // Extract hashtags
        for word in input.split_whitespace() {
            if word.starts_with('#') {
                tags.push(word.trim_start_matches('#').to_string());
            }
        }
        
        // Auto-tag based on content
        if input_lower.contains("meeting") {
            tags.push("meeting".to_string());
        }
        if input_lower.contains("review") {
            tags.push("review".to_string());
        }
        if input_lower.contains("sync") {
            tags.push("sync".to_string());
        }
        if input_lower.contains("standup") {
            tags.push("standup".to_string());
        }
        
        tags
    }

    fn detect_recurring(input: &str) -> Option<calendar_core::RecurrenceConfig> {
        let input_lower = input.to_lowercase();
        
        if input_lower.contains("daily") {
            return Some(calendar_core::RecurrenceConfig {
                frequency: calendar_core::RecurrenceFrequency::Daily,
                interval: 1,
                days_of_week: vec![],
                end_date: None,
                occurrences: None,
                except_dates: vec![],
            });
        }
        
        if input_lower.contains("weekly") || input_lower.contains("every week") {
            return Some(calendar_core::RecurrenceConfig {
                frequency: calendar_core::RecurrenceFrequency::Weekly,
                interval: 1,
                days_of_week: vec![],
                end_date: None,
                occurrences: None,
                except_dates: vec![],
            });
        }
        
        if input_lower.contains("monthly") {
            return Some(calendar_core::RecurrenceConfig {
                frequency: calendar_core::RecurrenceFrequency::Monthly,
                interval: 1,
                days_of_week: vec![],
                end_date: None,
                occurrences: None,
                except_dates: vec![],
            });
        }
        
        if input_lower.contains("yearly") {
            return Some(calendar_core::RecurrenceConfig {
                frequency: calendar_core::RecurrenceFrequency::Yearly,
                interval: 1,
                days_of_week: vec![],
                end_date: None,
                occurrences: None,
                except_dates: vec![],
            });
        }
        
        // Check for "every Monday", "every Friday", etc.
        if let Some(caps) = RECURRING_REGEX.captures(input) {
            if let Some(weekday_str) = caps.get(3) {
                if let Some(weekday) = Self::parse_weekday(weekday_str.as_str()) {
                    let weekday_num = weekday.number_from_monday() - 1; // Convert to 0-indexed (Mon = 0)
                    
                    return Some(calendar_core::RecurrenceConfig {
                        frequency: calendar_core::RecurrenceFrequency::Weekly,
                        interval: 1,
                        days_of_week: vec![weekday_num as u8],
                        end_date: None,
                        occurrences: None,
                        except_dates: vec![],
                    });
                }
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_today() {
        let parser = SimpleParser;
        let result = parser.parse("Meeting today at 2pm").unwrap();
        
        let today = Local::now().date_naive().format("%Y-%m-%d").to_string();
        assert_eq!(result.date, today);
        assert_eq!(result.time, Some("14:00".to_string()));
        assert!(result.event.contains("Meeting"));
    }
    
    #[test]
    fn test_parse_tomorrow() {
        let parser = SimpleParser;
        let result = parser.parse("Lunch tomorrow").unwrap();
        
        let tomorrow = (Local::now().date_naive() + Duration::days(1))
            .format("%Y-%m-%d").to_string();
        assert_eq!(result.date, tomorrow);
        assert_eq!(result.time, Some("12:00".to_string())); // "lunch" → 12:00
        assert_eq!(result.category, "social"); // "lunch" → social
    }
    
    #[test]
    fn test_parse_time_12hour_pm() {
        let parser = SimpleParser;
        let result = parser.parse("Event at 2pm").unwrap();
        assert_eq!(result.time, Some("14:00".to_string()));
    }
    
    #[test]
    fn test_parse_time_12hour_am() {
        let parser = SimpleParser;
        let result = parser.parse("Event at 9am").unwrap();
        assert_eq!(result.time, Some("09:00".to_string()));
    }
    
    #[test]
    fn test_parse_time_24hour() {
        let parser = SimpleParser;
        let result = parser.parse("Event at 14:30").unwrap();
        assert_eq!(result.time, Some("14:30".to_string()));
    }
    
    #[test]
    fn test_relative_time_morning() {
        let parser = SimpleParser;
        let result = parser.parse("Meeting in the morning").unwrap();
        assert_eq!(result.time, Some("09:00".to_string()));
    }
    
    #[test]
    fn test_relative_time_evening() {
        let parser = SimpleParser;
        let result = parser.parse("Dinner in the evening").unwrap();
        assert_eq!(result.time, Some("18:00".to_string()));
    }
    
    #[test]
    fn test_priority_urgent() {
        let parser = SimpleParser;
        let result = parser.parse("Urgent meeting tomorrow").unwrap();
        assert_eq!(result.priority, "urgent");
    }
    
    #[test]
    fn test_priority_deadline() {
        let parser = SimpleParser;
        let result = parser.parse("Project deadline next week").unwrap();
        assert_eq!(result.priority, "urgent");
    }
    
    #[test]
    fn test_category_work() {
        let parser = SimpleParser;
        let result = parser.parse("Team meeting tomorrow").unwrap();
        assert_eq!(result.category, "work");
    }
    
    #[test]
    fn test_category_social() {
        let parser = SimpleParser;
        let result = parser.parse("Coffee with Sarah").unwrap();
        assert_eq!(result.category, "social");
    }
    
    #[test]
    fn test_category_health() {
        let parser = SimpleParser;
        let result = parser.parse("Doctor appointment tomorrow").unwrap();
        assert_eq!(result.category, "health");
    }
    
    #[test]
    fn test_tag_extraction_hashtag() {
        let parser = SimpleParser;
        let result = parser.parse("Meeting tomorrow #important #urgent").unwrap();
        
        assert!(result.tags.contains(&"important".to_string()));
        assert!(result.tags.contains(&"urgent".to_string()));
    }
    
    #[test]
    fn test_auto_tag_meeting() {
        let parser = SimpleParser;
        let result = parser.parse("Team meeting tomorrow").unwrap();
        
        assert!(result.tags.contains(&"meeting".to_string()));
    }
    
    #[test]
    fn test_end_time_inferred() {
        let parser = SimpleParser;
        let result = parser.parse("Meeting at 2pm").unwrap();
        
        assert_eq!(result.time, Some("14:00".to_string()));
        assert_eq!(result.end_time, Some("15:00".to_string())); // +1 hour
    }
    
    #[test]
    fn test_recurring_daily() {
        let parser = SimpleParser;
        let result = parser.parse("Standup daily at 9am").unwrap();
        
        assert!(result.recurring.is_some());
        assert_eq!(result.recurring.unwrap().frequency, calendar_core::RecurrenceFrequency::Daily);
    }
    
    #[test]
    fn test_recurring_weekly() {
        let parser = SimpleParser;
        let result = parser.parse("Meeting every Monday at 2pm").unwrap();
        
        assert!(result.recurring.is_some());
        let rec = result.recurring.unwrap();
        assert_eq!(rec.frequency, calendar_core::RecurrenceFrequency::Weekly);
        assert!(!rec.days_of_week.is_empty());
    }
    
    #[test]
    fn test_recurring_monthly() {
        let parser = SimpleParser;
        let result = parser.parse("Report monthly").unwrap();
        
        assert!(result.recurring.is_some());
        assert_eq!(result.recurring.unwrap().frequency, calendar_core::RecurrenceFrequency::Monthly);
    }
}
