pub mod protocol;
pub mod message;

pub use protocol::{IpcRequest, IpcResponse, CalendarEventDto, generate_request_id};
