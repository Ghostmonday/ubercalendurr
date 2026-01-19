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
