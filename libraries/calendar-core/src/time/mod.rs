use chrono::{DateTime, Utc, Local, NaiveDate, NaiveTime};
use once_cell::sync::Lazy;

pub mod prelude {
    pub use super::{TimeParser, TimeZone, now_utc, today, format_date, parse_date_string};
}

static RELATIVE_DATE_PATTERN: Lazy<regex::Regex> = 
    Lazy::new(|| regex::Regex::new(r"^(today|tomorrow|yesterday|this week|this month|next week|next month)$").unwrap());

static DAY_OF_WEEK_PATTERN: Lazy<regex::Regex> = 
    Lazy::new(|| regex::Regex::new(r"^(monday|tuesday|wednesday|thursday|friday|saturday|sunday)$").unwrap());

pub struct TimeParser;

impl TimeParser {
    pub fn parse_relative_date(input: &str) -> Option<NaiveDate> {
        let normalized = input.to_lowercase().trim().to_string();
        let today = Local::now().date_naive();

        if normalized == "today" {
            Some(today)
        } else if normalized == "tomorrow" {
            Some(today.succ_opt()?)
        } else if normalized == "yesterday" {
            Some(today.pred_opt()?)
        } else if normalized.starts_with("next ") {
            let day_name = &normalized[5..];
            Self::parse_day_of_week(day_name).map(|d| {
                let mut date = today;
                while date.weekday() != d {
                    date = date.succ_opt().unwrap();
                }
                date
            })
        } else {
            None
        }
    }

    pub fn parse_day_of_week(input: &str) -> Option<chrono::Weekday> {
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

    pub fn parse_time(input: &str) -> Option<NaiveTime> {
        NaiveTime::parse_from_str(input, "%H:%M").ok()
            .or(NaiveTime::parse_from_str(input, "%I:%M %p").ok())
    }

    pub fn parse_date_string(input: &str) -> Option<String> {
        // Try ISO format first
        if DATE_PATTERN.is_match(input) {
            return Some(input.to_string());
        }

        // Try relative dates
        if let Some(date) = Self::parse_relative_date(input) {
            return Some(date.format("%Y-%m-%d").to_string());
        }

        // Try "next [day]"
        if let Some(rest) = input.strip_prefix("next ") {
            if let Some(weekday) = Self::parse_day_of_week(rest) {
                let today = Local::now().date_naive();
                let mut date = today;
                while date.weekday() != weekday {
                    date = date.succ_opt().unwrap();
                }
                return Some(date.format("%Y-%m-%d").to_string());
            }
        }

        None
    }
}

fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

fn today() -> NaiveDate {
    Local::now().date_naive()
}

fn format_date(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

static DATE_PATTERN: Lazy<regex::Regex> = 
    Lazy::new(|| regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());

pub enum TimeZone {
    Local,
    Utc,
}

impl TimeZone {
    pub fn now(&self) -> DateTime<chrono::FixedOffset> {
        match self {
            TimeZone::Local => Local::now().fixed_offset(),
            TimeZone::Utc => Utc::now().fixed_offset(),
        }
    }
}
