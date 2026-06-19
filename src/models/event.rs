use super::{GrampsDate, Handle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateEventRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<GrampsDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<Handle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_list: Option<Vec<Handle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_list: Option<Vec<Handle>>,
}
