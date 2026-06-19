use super::Handle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSourceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubinfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbrev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_list: Option<Vec<Handle>>,
}
