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
