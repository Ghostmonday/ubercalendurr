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
