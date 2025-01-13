use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::sysinfo::SysInfo;

/// Responses, either sent as is, or nested in StructuredResponse below
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "name", content = "data")]
pub enum Response {
    Photo(Photo),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
    pub image: String,
    pub timestamp: String,
    pub size_converted: usize,
    pub size_original: usize,
    pub pi_info: SysInfo,
}

/// These get sent to the websocket server when in structured_data mode,
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StructuredResponse {
    data: Option<Response>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Response>,
    unique: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<bool>,
}

impl StructuredResponse {
    /// Convert a ResponseMessage into a Tokio message of StructureResponse
    pub fn data(data: Response, unique: String, cache: Option<bool>) -> Message {
        let x = Self {
            data: Some(data),
            error: None,
            unique,
            cache,
        };
        Message::Text(serde_json::to_string(&x).unwrap_or_default().into())
    }

    /// Convert a ErrorResponse into a Tokio message of StructureResponse
    pub fn _error(data: Response, unique: String) -> Message {
        let x = Self {
            error: Some(data),
            data: None,
            unique,
            cache: Some(false),
        };
        Message::Text(serde_json::to_string(&x).unwrap_or_default().into())
    }
}
