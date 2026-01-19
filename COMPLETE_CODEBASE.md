# UberCalendurr - Complete Codebase

A revolutionary dual-interface calendar application with terminal-first design and AI-powered scheduling.

## Table of Contents

### 1. Workspace Configuration
- [Cargo.toml](#1-cargotoml) - Root workspace configuration

### 2. Libraries
#### 2.1 [calendar-core](#21-calendar-core) - Core data models
- [Cargo.toml](#211-cargotoml)
- [src/lib.rs](#212-srclibrs)
- [src/models/mod.rs](#213-srcmodelsmodrs)
- [src/errors.rs](#214-srcerrosrs)
- [src/validation.rs](#215-srcvalidationrs)
- [src/time/mod.rs](#216-srctimemodrs)

#### 2.2 [deepseek-client](#22-deepseek-client) - DeepSeek API integration
- [Cargo.toml](#221-cargotoml)
- [src/lib.rs](#222-srclibrs)
- [src/client.rs](#223-srcclientrs)
- [src/models/mod.rs](#224-srcmodelsmodrs)
- [src/prompts.rs](#225-srcpromptsrs)

#### 2.3 [storage-engine](#23-storage-engine) - SQLite storage
- [Cargo.toml](#231-cargotoml)
- [src/lib.rs](#232-srclibrs)
- [src/repository.rs](#233-srcrepositoryrs)
- [src/errors.rs](#234-srcerrosrs)
- [src/migrations.rs](#235-srcmigrationsrs)

#### 2.4 [IPC-primitives](#24-ipc-primitives) - IPC protocol
- [Cargo.toml](#241-cargotoml)
- [src/lib.rs](#242-srclibrs)
- [src/protocol.rs](#243-srcprotocolrs)
- [src/message.rs](#244-srcmessagers)

### 3. Binaries
#### 3.1 [calendar-widget](#31-calendar-widget) - Terminal widget
- [Cargo.toml](#311-cargotoml)
- [src/main.rs](#312-srcmainrs)
- [src/app.rs](#313-srcapprs)
- [src/config.rs](#314-srcconfigrs)
- [src/storage.rs](#315-srcstoragers)
- [src/api.rs](#316-srcapirs)
- [src/input.rs](#317-srcinputrs)
- [src/ipc.rs](#318-srcipc.rs)
- [src/ui.rs](#319-srcuirs)

#### 3.2 [calendar-gui](#32-calendar-gui) - GUI application
- [Cargo.toml](#321-cargotoml)
- [src/main.rs](#322-srcmainrs)

##### Frontend
- [frontend/package.json](#323-frontendpackagejson)
- [frontend/vite.config.ts](#324-frontendviteconfigts)
- [frontend/src/main.tsx](#325-frontendsrcmaintsx)
- [frontend/src/App.tsx](#326-frontendsrcapptsx)
- [frontend/src/index.css](#327-frontendsrcindexcss)
- [frontend/src/types/event.ts](#328-frontendsrctypeseventts)
- [frontend/src/utils/date.ts](#329-frontendsrcutilsdatets)
- [frontend/src/components/Calendar/CalendarGrid.tsx](#3210-frontendsrccomponentsCalendarCalendarGridtsx)
- [frontend/src/components/Terminal/TerminalInput.tsx](#3211-frontendsrccomponentsTerminalTerminalInputtsx)
- [frontend/src/components/Settings/SettingsPanel.tsx](#3212-frontendsrccomponentsSettingsSettingsPaneltsx)

---

# 1. Cargo.toml

```toml
[workspace]
members = [
    "binaries/calendar-widget",
    "binaries/calendar-gui",
    "libraries/calendar-core",
    "libraries/deepseek-client",
    "libraries/storage-engine",
    "libraries/IPC-primitives",
]
resolver = "2"
exclude = ["binaries/calendar-gui/frontend"]

[workspace.package]
authors = ["UberCalendurr Team"]
description = "Dual-interface calendar application with AI-powered scheduling"
repository = "https://github.com/Ghostmonday/ubercalendurr"
license = "MIT"
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde", "std"] }
regex = "1.10"
once_cell = "1.19"
dashmap = "6.0"
parking_lot = "0.12"
bincode = "1.3"
capnp = "0.20"
named_pipe = "0.5"
rusqlite = { version = "0.31", features = ["bundled", "trace"] }
reqwest = { version = "0.11", features = ["json", "tls"] }
tower = "0.4"
hyper = "1.0"
toml = "0.8"
directories = "5.0"
tracing = "0.1"
tracing-subscriber = "0.3"
env_logger = "0.10"

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = true
panic = "abort"

[profile.dev]
debug = true
split-debuginfo = "packed"
```

---

# 2. Libraries

## 2.1 calendar-core

### 2.1.1 Cargo.toml

```toml
[package]
name = "calendar-core"
version = "0.1.0"
edition = "2021"
description = "Core calendar data models and validation"
authors = ["UberCalendurr Team"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde", "std"] }
regex = "1.10"
thiserror = "1.0"
once_cell = "1.19"
```

### 2.1.2 src/lib.rs

```rust
pub mod models;
pub mod time;
pub mod errors;
pub mod validation;

pub use models::{CalendarEvent, Priority, Category, EventStatus, Visibility};
pub use errors::{CoreError, Result};
pub use validation::Validator;
```

### 2.1.3 src/models/mod.rs

```rust
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
```

### 2.1.4 src/errors.rs

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("JSON error: {0}")]
    JsonError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;

impl From<std::io::Error> for CoreError {
    fn from(e: std::io::Error) -> Self {
        CoreError::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(e: serde_json::Error) -> Self {
        CoreError::JsonError(e.to_string())
    }
}

impl From<uuid::Error> for CoreError {
    fn from(e: uuid::Error) -> Self {
        CoreError::ParseError(e.to_string())
    }
}
```

### 2.1.5 src/validation.rs

```rust
use regex::Regex;
use once_cell::sync::Lazy;

static TIME_PATTERN: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"^(?:[01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap());

static DATE_PATTERN: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());

pub struct Validator;

impl Validator {
    pub fn validate_time(time: &str) -> Result<(), String> {
        if !TIME_PATTERN.is_match(time) {
            Err(format!("Invalid time format: {}", time))
        } else {
            Ok(())
        }
    }

    pub fn validate_date(date: &str) -> Result<(), String> {
        if !DATE_PATTERN.is_match(date) {
            Err(format!("Invalid date format: {}", date))
        } else {
            Ok(())
        }
    }

    pub fn validate_hex_color(color: &str) -> Result<(), String> {
        if !color.starts_with('#') || color.len() != 7 {
            Err(format!("Invalid hex color: {}", color))
        } else {
            Ok(())
        }
    }

    pub fn sanitize_input(input: &str) -> String {
        input.trim().to_string()
    }
}
```

### 2.1.6 src/time/mod.rs

```rust
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
        if DATE_PATTERN.is_match(input) {
            return Some(input.to_string());
        }

        if let Some(date) = Self::parse_relative_date(input) {
            return Some(date.format("%Y-%m-%d").to_string());
        }

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
```

## 2.2 deepseek-client

### 2.2.1 Cargo.toml

```toml
[package]
name = "deepseek-client"
version = "0.1.0"
edition = "2021"
description = "DeepSeek API client for natural language parsing"
authors = ["UberCalendurr Team"]

[dependencies]
reqwest = { version = "0.11", features = ["json", "tls"] }
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
async-trait = "0.1"
futures = "0.3"
tracing = "0.1"
```

### 2.2.2 src/lib.rs

```rust
pub mod client;
pub mod models;
pub mod parser;
pub mod prompts;

pub use client::DeepSeekClient;
pub use models::{ChatMessage, MessageRole, ApiRequest, ApiResponse, Choice};
```

### 2.2.3 src/client.rs

```rust
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use anyhow::{Result, Context};
use tokio::sync::Mutex;
use crate::models::{ApiRequest, ApiResponse, ChatMessage};

#[derive(Clone, Debug)]
pub struct DeepSeekConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "deepseek-chat".to_string(),
            max_tokens: 1024,
            temperature: 0.3,
            timeout_seconds: 30,
            max_retries: 3,
        }
    }
}

pub struct DeepSeekClient {
    config: DeepSeekConfig,
    http_client: Client,
    rate_limiter: Arc<Mutex<RateLimiter>>,
}

struct RateLimiter {
    requests_per_minute: usize,
    current_count: usize,
    window_start: std::time::Instant,
}

impl DeepSeekClient {
    pub fn new(config: DeepSeekConfig) -> Result<Self> {
        let headers = Self::build_headers(&config.api_key)?;
        
        let http_client = ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            config,
            http_client,
            rate_limiter: Arc::new(Mutex::new(RateLimiter {
                requests_per_minute: 60,
                current_count: 0,
                window_start: std::time::Instant::now(),
            })),
        })
    }

    fn build_headers(api_key: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        
        let auth_value = format!("Bearer {}", api_key);
        headers.insert(AUTHORIZATION, auth_value.parse()?);
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        
        Ok(headers)
    }

    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<ApiResponse> {
        self.acquire_rate_limit().await;

        let request_body = ApiRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: false,
        };

        let mut retries = 0;
        let mut last_error = None;

        while retries <= self.config.max_retries {
            match self.send_request(&request_body).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    retries += 1;
                    
                    if retries <= self.config.max_retries {
                        tokio::time::sleep(Duration::from_millis(
                            500 * (2_u64.pow(retries - 1))
                        )).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| 
            anyhow::anyhow!("Max retries exceeded")
        ))
    }

    async fn send_request(
        &self,
        request: &ApiRequest
    ) -> Result<ApiResponse> {
        let response = self.http_client
            .post("https://api.deepseek.com/v1/chat/completions")
            .json(request)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let error_body = response.text().await?;
            return Err(anyhow::anyhow!(
                "API error ({}): {}", 
                response.status(), 
                error_body
            ));
        }

        let response_body: ApiResponse = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(response_body)
    }

    async fn acquire_rate_limit(&self) {
        let mut limiter = self.rate_limiter.lock().await;
        
        let elapsed = limiter.window_start.elapsed();
        if elapsed > Duration::from_secs(60) {
            limiter.current_count = 0;
            limiter.window_start = std::time::Instant::now();
        }

        if limiter.current_count >= limiter.requests_per_minute {
            let wait_time = Duration::from_secs(60) - elapsed;
            tokio::time::sleep(wait_time).await;
            limiter.current_count = 0;
            limiter.window_start = std::time::Instant::now();
        }

        limiter.current_count += 1;
    }
}
```

### 2.2.4 src/models/mod.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Function,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
    pub temperature: f32,
    pub stream: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEventOutput {
    pub event: String,
    pub date: String,
    pub time: Option<String>,
    pub end_time: Option<String>,
    pub notes: Option<String>,
    pub priority: String,
    pub category: String,
    pub recurring: Option<RecurrenceOutput>,
    pub reminder: Option<ReminderOutput>,
    pub location: Option<LocationOutput>,
    pub tags: Vec<String>,
    pub clarification_questions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrenceOutput {
    pub frequency: String,
    pub interval: u32,
    pub days_of_week: Vec<u8>,
    pub end_date: Option<String>,
    pub occurrences: Option<u32>,
    pub except_dates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderOutput {
    pub minutes_before: u32,
    pub repeat_minutes: Option<u32>,
    pub max_reminders: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationOutput {
    pub location_type: String,
    pub address: String,
}
```

### 2.2.5 src/prompts.rs

```rust
use crate::models::ChatMessage;

pub struct PromptTemplates {
    extraction_system: String,
}

impl PromptTemplates {
    pub fn new() -> Self {
        Self {
            extraction_system: Self::build_extraction_system_prompt(),
        }
    }

    fn build_extraction_system_prompt() -> String {
        r#"You are a calendar assistant that extracts structured event information from natural language text.

## Output Format
You must output a JSON object:
{
    "event": "Event title",
    "date": "YYYY-MM-DD",
    "time": "HH:MM or null",
    "endTime": "HH:MM or null",
    "notes": "Additional details or null",
    "priority": "low|medium|high|urgent",
    "category": "work|personal|health|social|finance|education|other",
    "recurring": null or {...},
    "reminder": null or {...},
    "location": null or {...},
    "tags": [],
    "clarificationQuestions": []
}

## Rules
1. Extract event title - focus on the "what"
2. Dates in YYYY-MM-DD format:
   - "today" = current date
   - "tomorrow" = current date + 1
   - "next [day]" = upcoming occurrence
3. Times in 24-hour HH:MM format
4. Infer defaults:
   - Lunch = 12:00
   - Morning meetings = 9:00
   - Evening events = 18:00
5. Priority:
   - "important", "critical", "urgent" → high/urgent
   - Default: "medium"
6. Categories:
   - Work terms → work
   - Medical terms → health
   - Money terms → finance
   - Social terms → social
   - Learning terms → education
   - Default: "personal"

## Ambiguity Handling
If unclear, set field to null and add to "clarificationQuestions":
- "I see 'lunch next week'—which day works best?"
- "What time should I schedule this?"
- "Should this be recurring?"

Output ONLY valid JSON:"#.to_string()
    }

    pub fn build_extraction_prompt(&self, user_input: &str) -> Vec<ChatMessage> {
        vec![
            ChatMessage {
                role: crate::models::MessageRole::System,
                content: self.extraction_system.clone(),
            },
            ChatMessage {
                role: crate::models::MessageRole::User,
                content: user_input.to_string(),
            },
        ]
    }
}
```

## 2.3 storage-engine

### 2.3.1 Cargo.toml

```toml
[package]
name = "storage-engine"
version = "0.1.0"
edition = "2021"
description = "SQLite storage layer for calendar events"
authors = ["UberCalendurr Team"]

[dependencies]
rusqlite = { version = "0.31", features = ["bundled", "trace"] }
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
once_cell = "1.19"
parking_lot = "0.12"
```

### 2.3.2 src/lib.rs

```rust
pub mod repository;
pub mod migrations;
pub mod errors;

pub use repository::CalendarRepository;
pub use errors::{StorageError, Result};
```

### 2.3.3 src/repository.rs

```rust
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

        conn.execute("CREATE INDEX IF NOT EXISTS idx_events_date ON events(date)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_events_category ON events(category)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_events_priority ON events(priority)", [])?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<CalendarEvent>> {
        let conn = self.connection.lock().await;
        
        let mut stmt = conn.prepare_cached("SELECT * FROM events WHERE id = ?1")?;

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
```

### 2.3.4 src/errors.rs

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query failed: {0}")]
    QueryFailed(String),

    #[error("Migration failed: {0}")]
    MigrationFailed(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Not found")]
    NotFound,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;
```

### 2.3.5 src/migrations.rs

```rust
pub struct Migrations;

impl Migrations {
    pub fn get_migrations() -> Vec<&'static str> {
        vec![
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
```

## 2.4 IPC-primitives

### 2.4.1 Cargo.toml

```toml
[package]
name = "IPC-primitives"
version = "0.1.0"
edition = "2021"
description = "IPC protocol definitions for inter-process communication"
authors = ["UberCalendurr Team"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.6", features = ["v4", "serde"] }
thiserror = "1.0"
bincode = "1.3"
```

### 2.4.2 src/lib.rs

```rust
pub mod protocol;
pub mod message;

pub use protocol::{IpcRequest, IpcResponse, CalendarEventDto, generate_request_id};
```

### 2.4.3 src/protocol.rs

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use calendar_core::{CalendarEvent, Category, Priority, EventStatus, Visibility};

pub const IPC_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum IpcRequest {
    GetEvents { start_date: String, end_date: String },
    GetEvent { event_id: String },
    CreateEvent { event: CalendarEventDto },
    UpdateEvent { event_id: String, event: CalendarEventDto },
    DeleteEvent { event_id: String },
    SearchEvents { query: String },
    GetTodayEvents,
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum IpcResponse {
    Success { request_id: String, payload: serde_json::Value },
    Error { request_id: String, code: String, message: String },
    Ack { request_id: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEventDto {
    pub id: Option<String>,
    pub date: String,
    pub time: Option<String>,
    pub end_time: Option<String>,
    pub event: String,
    pub notes: Option<String>,
    pub priority: String,
    pub category: String,
    pub color: Option<String>,
    pub tags: Vec<String>,
}

pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

impl IpcRequest {
    pub fn to_message(&self, request_id: String) -> crate::message::IpcMessage {
        let payload = serde_json::to_string(&self)
            .unwrap_or_default();
        
        crate::message::IpcMessage {
            version: IPC_VERSION,
            message_type: crate::message::MessageType::Request,
            request_id,
            payload,
        }
    }
}

impl IpcResponse {
    pub fn from_message(message: &crate::message::IpcMessage) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&message.payload)
    }

    pub fn success<T: Serialize>(
        request_id: String, 
        data: &T
    ) -> Result<crate::message::IpcMessage, serde_json::Error> {
        let payload = serde_json::to_string(&Self::Success {
            request_id,
            payload: serde_json::to_value(data)?,
        })?;

        Ok(crate::message::IpcMessage {
            version: IPC_VERSION,
            message_type: crate::message::MessageType::Response,
            request_id,
            payload,
        })
    }

    pub fn error(
        request_id: String, 
        code: String, 
        message: String
    ) -> Result<crate::message::IpcMessage, serde_json::Error> {
        let payload = serde_json::to_string(&Self::Error {
            request_id,
            code,
            message,
        })?;

        Ok(crate::message::IpcMessage {
            version: IPC_VERSION,
            message_type: crate::message::MessageType::Response,
            request_id,
            payload,
        })
    }
}
```

### 2.4.4 src/message.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Request,
    Response,
    Notification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub version: u32,
    pub message_type: MessageType,
    pub request_id: String,
    pub payload: String,
}

impl IpcMessage {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}
```

---

# 3. Binaries

## 3.1 calendar-widget

### 3.1.1 Cargo.toml

```toml
[package]
name = "calendar-widget"
version = "0.1.0"
edition = "2021"
description = "Terminal widget for UberCalendurr"
authors = ["UberCalendurr Team"]

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde", "std"] }
regex = "1.10"
once_cell = "1.19"
directories = "5.0"
tracing = "0.1"
tracing-subscriber = "0.3"

calendar-core = { path = "../../libraries/calendar-core" }
deepseek-client = { path = "../../libraries/deepseek-client" }
storage-engine = { path = "../../libraries/storage-engine" }
IPC-primitives = { path = "../../libraries/IPC-primitives" }

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser", "shell32"] }
```

### 3.1.2 src/main.rs

```rust
#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Result, Context};
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;

mod app;
mod config;
mod storage;
mod api;
mod input;
mod ui;
mod ipc;

use crate::app::App;
use crate::config::Settings;
use crate::storage::Repository;
use crate::api::DeepSeekClient;

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    info!("Starting UberCalendurr Widget {}", env!("CARGO_PKG_VERSION"));

    let config_path = get_config_path()?;
    let settings = Settings::load(&config_path)
        .context("Failed to load settings")?;

    let state = Arc::new(AppState::new(&settings)?);

    let mut app = App::new(state.clone())?;

    info!("UberCalendurr Widget initialized successfully");

    app.run()?;

    state.save()?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = directories::UserConfigDir
        .ok_or_else(|| anyhow::anyhow!("Unable to determine config directory"))?
        .join("ubercalendurr");
    
    std::fs::create_dir_all(&config_dir)
        .context("Failed to create config directory")?;

    Ok(config_dir.join("settings.toml"))
}

#[derive(Clone)]
struct AppState {
    settings: Arc<Settings>,
    repository: Arc<Repository>,
    deepseek_client: Arc<DeepSeekClient>,
    input_buffer: Arc<std::sync::RwLock<String>>,
    processing_state: Arc<std::sync::RwLock<ProcessingState>>,
}

#[derive(Debug, Clone, PartialEq)]
enum ProcessingState {
    Idle,
    Processing,
    Complete,
    Error(String),
}

impl AppState {
    fn new(settings: &Settings) -> Result<Self> {
        let repository = Arc::new(Repository::new(&settings.database_path)?);
        let deepseek_client = Arc::new(DeepSeekClient::new(&settings.deepseek_api_key)?);

        Ok(Self {
            settings: Arc::new(settings.clone()),
            repository,
            deepseek_client,
            input_buffer: Arc::new(std::sync::RwLock::new(String::new())),
            processing_state: Arc::new(std::sync::RwLock::new(ProcessingState::Idle)),
        })
    }

    fn save(&self) -> Result<()> {
        self.settings.save()
    }
}
```

### 3.1.3 src/app.rs

```rust
use std::sync::Arc;
use crate::AppState;

pub struct App {
    state: Arc<AppState>,
}

impl App {
    pub fn new(state: Arc<AppState>) -> Result<Self, std::io::Error> {
        Ok(Self { state })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        println!("UberCalendurr Widget v0.1.0");
        println!("Type /help for available commands");
        println!();

        self.run_interactive()
    }

    fn run_interactive(&mut self) -> Result<(), std::io::Error> {
        use std::io::{self, Write};

        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            print!("> ");
            io::stdout().flush()?;

            input.clear();
            let bytes_read = stdin.read_line(&mut input)?;

            if bytes_read == 0 {
                break;
            }

            let input = input.trim().to_string();

            if input.is_empty() {
                continue;
            }

            if input == "/exit" || input == "/quit" {
                break;
            }

            println!("Processing: {}", input);
        }

        Ok(())
    }
}
```

### 3.1.4 src/config.rs

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};
use directories::UserConfigDir;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub display: DisplaySettings,
    pub hotkey: HotkeySettings,
    pub api: ApiSettings,
    pub notifications: NotificationSettings,
    pub appearance: AppearanceSettings,
    pub database_path: PathBuf,
    pub deepseek_api_key: String,
    pub debug_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            display: DisplaySettings::default(),
            hotkey: HotkeySettings::default(),
            api: ApiSettings::default(),
            notifications: NotificationSettings::default(),
            appearance: AppearanceSettings::default(),
            database_path: PathBuf::from("calendar.db"),
            deepseek_api_key: String::new(),
            debug_mode: false,
        }
    }
}

impl Settings {
    pub fn load(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            let default = Self::default();
            default.save_to(path)?;
            return Ok(default);
        }

        let content = fs::read_to_string(path)
            .context("Failed to read settings file")?;

        let settings: Settings = toml::from_str(&content)
            .context("Failed to parse settings")?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = get_config_dir()?;
        std::fs::create_dir_all(&config_dir)?;
        let path = config_dir.join("settings.toml");
        self.save_to(&path)
    }

    pub fn save_to(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize settings")?;
        std::fs::write(path, content)
            .context("Failed to write settings file")?;
        Ok(())
    }
}

fn get_config_dir() -> Result<PathBuf> {
    UserConfigDir::ok_or_else(|| anyhow::anyhow!("No config directory"))
        .map(|d| d.to_path_buf())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplaySettings {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub opacity: f64,
    pub always_on_top: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 500,
            height: 400,
            opacity: 0.95,
            always_on_top: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HotkeySettings {
    pub activate: String,
    pub toggle: String,
}

impl Default for HotkeySettings {
    fn default() -> Self {
        Self {
            activate: "Ctrl+Shift+C".to_string(),
            toggle: "Ctrl+Shift+H".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiSettings {
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            model: "deepseek-chat".to_string(),
            max_tokens: 1024,
            temperature: 0.3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotificationSettings {
    pub enabled: bool,
    pub default_reminder_minutes: u32,
    pub play_sound: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            default_reminder_minutes: 15,
            play_sound: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppearanceSettings {
    pub theme: String,
    pub font_family: String,
    pub font_size: u32,
    pub show_hints: bool,
    pub show_speech_hint: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_family: "Cascadia Code".to_string(),
            font_size: 14,
            show_hints: true,
            show_speech_hint: true,
        }
    }
}
```

### 3.1.5 src/storage.rs

```rust
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
```

### 3.1.6 src/api.rs

```rust
pub use deepseek_client::{DeepSeekClient, DeepSeekConfig};
```

### 3.1.7 src/input.rs

```rust
use std::collections::VecDeque;
use crate::input::parser::InputParser;

pub mod parser {
    use regex::Regex;
    use once_cell::sync::Lazy;

    pub struct InputParser;

    impl InputParser {
        pub fn is_json(&self, input: &str) -> bool {
            let trimmed = input.trim();
            trimmed.starts_with('{') && trimmed.ends_with('}')
        }

        pub fn is_command(&self, input: &str) -> bool {
            input.starts_with('/')
        }

        pub fn parse_command(&self, input: &str) -> Option<Command> {
            let parts: Vec<&str> = input.splitn(2, ' ').collect();
            match parts[0] {
                "/help" => Some(Command::Help),
                "/today" => Some(Command::ShowToday),
                "/search" => Some(Command::Search(parts.get(1).map(|s| s.to_string()).unwrap_or_default())),
                "/settings" => Some(Command::Settings),
                "/clear" => Some(Command::Clear),
                "/exit" | "/quit" => Some(Command::Exit),
                _ => None,
            }
        }
    }

    pub enum Command<'a> {
        Help,
        ShowToday,
        Search(String),
        Settings,
        Clear,
        Exit,
    }
}

pub struct InputHandler {
    parser: InputParser,
    input_history: VecDeque<String>,
    history_position: Option<usize>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            parser: InputParser,
            input_history: VecDeque::with_capacity(100),
            history_position: None,
        }
    }

    pub fn handle_input(&mut self, input: &str) -> InputResult {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return InputResult::Clear;
        }

        if let Some(command) = self.parser.parse_command(trimmed) {
            return self.execute_command(command);
        }

        if self.parser.is_json(trimmed) {
            return InputResult::Info("JSON input detected".to_string());
        }

        InputResult::Processing(trimmed.to_string())
    }

    fn execute_command(&self, command: parser::Command<'_>) -> InputResult {
        match command {
            parser::Command::Help => InputResult::ShowHelp,
            parser::Command::ShowToday => InputResult::Info("Showing today's events".to_string()),
            parser::Command::Search(query) => InputResult::Search(query),
            parser::Command::Settings => InputResult::OpenSettings,
            parser::Command::Clear => InputResult::Clear,
            parser::Command::Exit => InputResult::Exit,
        }
    }

    pub fn history_up(&mut self) -> Option<&str> {
        if let Some(pos) = self.history_position {
            if pos > 0 {
                self.history_position = Some(pos - 1);
                return self.input_history.get(pos - 1).map(|s| s.as_str());
            }
        }
        None
    }

    pub fn history_down(&mut self) -> Option<&str> {
        if let Some(pos) = self.history_position {
            if pos < self.input_history.len() - 1 {
                self.history_position = Some(pos + 1);
                return self.input_history.get(pos + 1).map(|s| s.as_str());
            } else {
                self.history_position = None;
            }
        }
        None
    }
}

pub enum InputResult {
    Processing(String),
    ShowHelp,
    Info(String),
    Search(String),
    OpenSettings,
    Clear,
    Exit,
    Error(String),
}
```

### 3.1.8 src/ipc.rs

```rust
pub use IPC-primitives::{IpcRequest, IpcResponse};
```

### 3.1.9 src/ui.rs

```rust
pub mod components;
```

## 3.2 calendar-gui

### 3.2.1 Cargo.toml

```toml
[package]
name = "calendar-gui"
version = "0.1.0"
edition = "2021"
description = "GUI application for UberCalendurr"
authors = ["UberCalendurr Team"]

[build-dependencies]
tauri-build = "1.5"

[dependencies]
tauri = { version = "1.6", features = ["shell-open"] }
tauri-build = "1.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

calendar-core = { path = "../../libraries/calendar-core" }
IPC-primitives = { path = "../../libraries/IPC-primitives" }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]

[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```

### 3.2.2 src/main.rs

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to UberCalendurr!", name)
}

#[tauri::command]
fn get_events(start_date: String, end_date: String) -> Result<Vec<calendar_core::CalendarEvent>, String> {
    Ok(vec![])
}

#[tauri::command]
fn create_event(event: serde_json::Value) -> Result<calendar_core::CalendarEvent, String> {
    Ok(calendar_core::CalendarEvent::new(
        event.get("event").and_then(|v| v.as_str()).unwrap_or("New Event").to_string(),
        event.get("date").and_then(|v| v.as_str()).unwrap_or("2024-01-01").to_string(),
    ))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_events, create_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3.2.3 frontend/package.json

```json
{
  "name": "ubercalendurr-frontend",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "zustand": "^4.4.7",
    "date-fns": "^3.0.6",
    "uuid": "^9.0.0",
    "framer-motion": "^10.17.4",
    "lucide-react": "^0.303.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.43",
    "@types/react-dom": "^18.2.17",
    "@types/uuid": "^9.0.7",
    "@typescript-eslint/eslint-plugin": "^6.14.0",
    "@typescript-eslint/parser": "^6.14.0",
    "@vitejs/plugin-react": "^4.2.1",
    "autoprefixer": "^10.4.16",
    "eslint": "^8.55.0",
    "eslint-plugin-react-hooks": "^4.6.0",
    "eslint-plugin-react-refresh": "^0.4.5",
    "postcss": "^8.4.32",
    "prettier": "^3.1.1",
    "tailwindcss": "^3.4.0",
    "typescript": "^5.3.3",
    "vite": "^5.0.8"
  }
}
```

### 3.2.4 frontend/vite.config.ts

```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@components': path.resolve(__dirname, './src/components'),
      '@hooks': path.resolve(__dirname, './src/hooks'),
      '@store': path.resolve(__dirname, './src/store'),
      '@types': path.resolve(__dirname, './src/types'),
      '@utils': path.resolve(__dirname, './src/utils'),
    },
  },
  server: {
    port: 3000,
  },
  build: {
    outDir: 'dist',
    sourcemap: false,
  },
});
```

### 3.2.5 frontend/src/main.tsx

```typescript
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
```

### 3.2.6 frontend/src/App.tsx

```typescript
import React, { useState, useEffect } from 'react';
import { CalendarGrid } from './components/Calendar/CalendarGrid';
import { TerminalInput } from './components/Terminal/TerminalInput';
import { SettingsPanel } from './components/Settings/SettingsPanel';
import { CalendarEvent } from './types/event';
import { invoke } from '@tauri-apps/api/tauri';

function App() {
  const [currentDate, setCurrentDate] = useState(new Date());
  const [events, setEvents] = useState<CalendarEvent[]>([]);
  const [showSettings, setShowSettings] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadEvents();
  }, [currentDate]);

  const loadEvents = async () => {
    setLoading(true);
    try {
      setEvents([
        {
          id: '1',
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          date: new Date().toISOString().split('T')[0],
          time: '09:00',
          endTime: '10:00',
          event: 'Team Standup',
          notes: 'Weekly team meeting',
          priority: 'medium',
          category: 'work',
          color: '#3B82F6',
          tags: ['team'],
          status: 'confirmed',
          visibility: 'private',
          recurring: null,
          reminder: null,
          location: null,
          metadata: {},
        },
        {
          id: '2',
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          date: new Date().toISOString().split('T')[0],
          time: '12:00',
          endTime: '13:00',
          event: 'Lunch with Sarah',
          notes: '',
          priority: 'low',
          category: 'personal',
          color: '#10B981',
          tags: ['sarah'],
          status: 'confirmed',
          visibility: 'private',
          recurring: null,
          reminder: null,
          location: null,
          metadata: {},
        },
      ]);
    } catch (error) {
      console.error('Failed to load events:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100">
      <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <h1 className="text-xl font-bold">UberCalendurr</h1>
            <span className="text-sm text-gray-500">AI-Powered Calendar</span>
          </div>
          <button
            onClick={() => setShowSettings(!showSettings)}
            className="p-2 hover:bg-gray-700 rounded-lg transition-colors"
          >
            Settings
          </button>
        </div>
      </header>

      <main className="p-6">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2">
            <CalendarGrid
              onEventClick={(event) => console.log('Event clicked:', event)}
              onDateClick={(date) => console.log('Date clicked:', date)}
            />
          </div>
          <div className="lg:col-span-1">
            <TerminalInput
              onEventCreated={(event) => {
                console.log('Event created:', event);
                loadEvents();
              }}
            />
          </div>
        </div>
      </main>

      {showSettings && (
        <SettingsPanel onClose={() => setShowSettings(false)} />
      )}
    </div>
  );
}

export default App;
```

### 3.2.7 frontend/src/index.css

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  font-family: 'Segoe UI', system-ui, sans-serif;
  line-height: 1.5;
  font-weight: 400;
  color-scheme: dark;
  background-color: #1a1b1e;
  color: #d4d4d4;
}

body {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
}

@layer components {
  .btn-primary {
    @apply px-4 py-2 bg-primary-600 hover:bg-primary-700 rounded-lg font-medium transition-colors;
  }

  .btn-secondary {
    @apply px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg font-medium transition-colors;
  }

  .input-field {
    @apply w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-primary-500;
  }

  .card {
    @apply bg-gray-800 border border-gray-700 rounded-xl p-4;
  }
}
```

### 3.2.8 frontend/src/types/event.ts

```typescript
import { z } from 'zod';

const prioritySchema = z.enum(['low', 'medium', 'high', 'urgent']);
const categorySchema = z.enum(['work', 'personal', 'health', 'social', 'finance', 'education', 'other']);

export const calendarEventSchema = z.object({
  id: z.string().uuid(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
  date: z.string().regex(/^\d{4}-\d{2}-\d{2}$/),
  time: z.string().regex(/^\d{2}:\d{2}$/).optional(),
  endTime: z.string().regex(/^\d{2}:\d{2}$/).optional(),
  event: z.string().min(1).max(500),
  notes: z.string().max(5000).optional(),
  priority: prioritySchema.default('medium'),
  category: categorySchema.default('other'),
  color: z.string().regex(/^#[0-9A-Fa-f]{6}$/).optional(),
  tags: z.array(z.string()).default([]),
  status: z.enum(['tentative', 'confirmed', 'cancelled', 'completed']).default('confirmed'),
  visibility: z.enum(['public', 'private']).default('private'),
  recurring: z.any().optional(),
  reminder: z.any().optional(),
  location: z.any().optional(),
  metadata: z.record(z.unknown()).default({}),
});

export type CalendarEvent = z.infer<typeof calendarEventSchema>;
export type Priority = z.infer<typeof prioritySchema>;
export type Category = z.infer<typeof categorySchema>;

export const categoryInfo: Record<Category, { name: string; color: string; icon: string }> = {
  work: { name: 'Work', color: '#3B82F6', icon: '💼' },
  personal: { name: 'Personal', color: '#10B981', icon: '🏠' },
  health: { name: 'Health', color: '#EF4444', icon: '❤️' },
  social: { name: 'Social', color: '#8B5CF6', icon: '👥' },
  finance: { name: 'Finance', color: '#F59E0B', icon: '💰' },
  education: { name: 'Education', color: '#6366F1', icon: '📚' },
  other: { name: 'Other', color: '#6B7280', icon: '📌' },
};

export const priorityInfo: Record<Priority, { level: number; label: string; emoji: string }> = {
  low: { level: 0, label: 'Low', emoji: '🟢' },
  medium: { level: 1, label: 'Medium', emoji: '🟡' },
  high: { level: 2, label: 'High', emoji: '🟠' },
  urgent: { level: 3, label: 'Urgent', emoji: '🔴' },
};
```

### 3.2.9 frontend/src/utils/date.ts

```typescript
interface CalendarDay {
  date: Date;
  isCurrentMonth: boolean;
  isToday: boolean;
}

export function generateCalendarDays(year: number, month: number): CalendarDay[] {
  const days: CalendarDay[] = [];
  const firstDay = new Date(year, month, 1);
  const lastDay = new Date(year, month + 1, 0);
  const today = new Date();

  for (let i = startDayOfWeek - 1; i >= 0; i--) {
    days.push({
      date: new Date(year, month - 1, prevMonthLastDay - i),
      isCurrentMonth: false,
      isToday: false,
    });
  }

  for (let day = 1; day <= lastDay.getDate(); day++) {
    const date = new Date(year, month, day);
    days.push({
      date,
      isCurrentMonth: true,
      isToday: date.toDateString() === today.toDateString(),
    });
  }

  const remainingDays = 42 - days.length;
  for (let day = 1; day <= remainingDays; day++) {
    days.push({
      date: new Date(year, month + 1, day),
      isCurrentMonth: false,
      isToday: false,
    });
  }

  return days;
}

export function formatDate(date: Date): string {
  return date.toISOString().split('T')[0];
}

export function parseDateString(dateStr: string): Date | null {
  const parsed = new Date(dateStr);
  return isNaN(parsed.getTime()) ? null : parsed;
}
```

### 3.2.10 frontend/src/components/Calendar/CalendarGrid.tsx

```typescript
import React, { useMemo } from 'react';
import { CalendarEvent } from '../../types/event';
import { generateCalendarDays } from '../../utils/date';

interface CalendarGridProps {
  onEventClick?: (event: CalendarEvent) => void;
  onDateClick?: (date: Date) => void;
}

export const CalendarGrid: React.FC<CalendarGridProps> = ({ onEventClick, onDateClick }) => {
  const currentDate = new Date();
  const calendarDays = useMemo(() => {
    return generateCalendarDays(currentDate.getFullYear(), currentDate.getMonth());
  }, [currentDate]);

  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

  return (
    <div className="calendar-grid bg-gray-800 rounded-xl overflow-hidden">
      <div className="flex items-center justify-between px-4 py-3 bg-gray-700">
        <button className="p-2 hover:bg-gray-600 rounded-lg">←</button>
        <h2 className="text-lg font-semibold">
          {currentDate.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
        </h2>
        <button className="p-2 hover:bg-gray-600 rounded-lg">→</button>
      </div>

      <div className="grid grid-cols-7 bg-gray-700 border-b border-gray-600">
        {dayNames.map(day => (
          <div key={day} className="px-2 py-2 text-center text-sm font-medium text-gray-400">
            {day}
          </div>
        ))}
      </div>

      <div className="grid grid-cols-7">
        {calendarDays.map((day, index) => {
          const isToday = day.isToday;
          const isCurrentMonth = day.isCurrentMonth;

          return (
            <div
              key={index}
              className={`
                min-h-[100px] p-2 border border-gray-700 cursor-pointer
                ${isCurrentMonth ? 'bg-gray-800' : 'bg-gray-900'}
                ${isToday ? 'ring-2 ring-primary-500' : ''}
                hover:bg-gray-750 transition-colors
              `}
              onClick={() => onDateClick?.(day.date)}
            >
              <div className={`
                text-sm font-medium mb-1
                ${isToday ? 'text-primary-500' : isCurrentMonth ? 'text-gray-200' : 'text-gray-600'}
              `}>
                {day.date.getDate()}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
```

### 3.2.11 frontend/src/components/Terminal/TerminalInput.tsx

```typescript
import React, { useState } from 'react';
import { CalendarEvent } from '../../types/event';

interface TerminalInputProps {
  onEventCreated?: (event: CalendarEvent) => void;
}

export const TerminalInput: React.FC<TerminalInputProps> = ({ onEventCreated }) => {
  const [input, setInput] = useState('');
  const [processing, setProcessing] = useState(false);
  const [suggestion, setSuggestion] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim()) return;

    setProcessing(true);
    setSuggestion(null);

    setTimeout(() => {
      const newEvent: CalendarEvent = {
        id: crypto.randomUUID(),
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        date: new Date().toISOString().split('T')[0],
        time: '14:00',
        endTime: '15:00',
        event: input,
        notes: '',
        priority: 'medium',
        category: 'personal',
        color: '#10B981',
        tags: [],
        status: 'confirmed',
        visibility: 'private',
        recurring: null,
        reminder: null,
        location: null,
        metadata: {},
      };

      onEventCreated?.(newEvent);
      setInput('');
      setProcessing(false);
      setSuggestion('Event created successfully!');
      setTimeout(() => setSuggestion(null), 3000);
    }, 1500);
  };

  return (
    <div className="bg-gray-800 rounded-xl p-4 border border-gray-700">
      <div className="flex items-center gap-2 mb-4">
        <span className="text-green-500">›</span>
        <span className="font-mono text-sm text-gray-400">Quick Entry</span>
      </div>

      <form onSubmit={handleSubmit}>
        <div className="relative">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder="Type an event... (e.g., 'Meeting tomorrow at 2pm')"
            className="w-full h-32 bg-gray-900 border border-gray-700 rounded-lg p-3 font-mono text-sm text-gray-200 placeholder-gray-600 focus:outline-none focus:border-primary-500 resize-none"
            disabled={processing}
          />
          
          <div className="absolute bottom-3 right-3 flex items-center gap-2">
            {processing && (
              <span className="text-xs text-yellow-500 animate-pulse">Processing...</span>
            )}
            <button
              type="submit"
              disabled={!input.trim() || processing}
              className="px-3 py-1 bg-primary-600 hover:bg-primary-700 disabled:bg-gray-700 disabled:text-gray-500 rounded text-sm font-medium transition-colors"
            >
              Enter
            </button>
          </div>
        </div>
      </form>

      {suggestion && (
        <div className="mt-3 p-2 bg-green-900/50 border border-green-700 rounded text-green-400 text-sm">
          {suggestion}
        </div>
      )}

      <div className="mt-4 text-xs text-gray-500">
        <p>Tips:</p>
        <ul className="list-disc list-inside mt-1 space-y-1">
          <li>Type naturally, e.g., "Lunch with John next Tuesday at noon"</li>
          <li>Use JSON for precise entries</li>
          <li>Commands: /help, /today, /search</li>
        </ul>
      </div>
    </div>
  );
};
```

### 3.2.12 frontend/src/components/Settings/SettingsPanel.tsx

```typescript
import React from 'react';

interface SettingsPanelProps {
  onClose: () => void;
}

export const SettingsPanel: React.FC<SettingsPanelProps> = ({ onClose }) => {
  const [apiKey, setApiKey] = React.useState('');
  const [theme, setTheme] = React.useState('dark');

  const handleSave = () => {
    console.log('Saving settings...');
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-xl w-full max-w-md mx-4 border border-gray-700">
        <div className="flex items-center justify-between px-6 py-4 border-b border-gray-700">
          <h2 className="text-lg font-semibold">Settings</h2>
          <button onClick={onClose} className="p-2 hover:bg-gray-700 rounded-lg transition-colors">
            ✕
          </button>
        </div>

        <div className="p-6 space-y-6">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              DeepSeek API Key
            </label>
            <input
              type="password"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              placeholder="Enter your API key"
              className="w-full px-4 py-2 bg-gray-900 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-primary-500"
            />
            <p className="mt-1 text-xs text-gray-500">
              Get your API key from platform.deepseek.com
            </p>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">Theme</label>
            <div className="flex gap-3">
              {['dark', 'light'].map((t) => (
                <button
                  key={t}
                  onClick={() => setTheme(t)}
                  className={`px-4 py-2 rounded-lg font-medium capitalize transition-colors ${
                    theme === t ? 'bg-primary-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                  }`}
                >
                  {t}
                </button>
              ))}
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">Activation Hotkey</label>
            <input
              type="text"
              defaultValue="Ctrl+Shift+C"
              className="w-full px-4 py-2 bg-gray-900 border border-gray-700 rounded-lg text-white focus:outline-none focus:border-primary-500"
            />
          </div>
        </div>

        <div className="flex justify-end gap-3 px-6 py-4 border-t border-gray-700">
          <button onClick={onClose} className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg font-medium transition-colors">
            Cancel
          </button>
          <button onClick={handleSave} className="px-4 py-2 bg-primary-600 hover:bg-primary-700 rounded-lg font-medium transition-colors">
            Save
          </button>
        </div>
      </div>
    </div>
  );
};
```

---

*Generated on: 2026-01-18*  
*Repository: https://github.com/Ghostmonday/ubercalendurr*
