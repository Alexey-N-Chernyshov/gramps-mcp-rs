use super::Handle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaceName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePlaceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<PlaceName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_list: Option<Vec<Handle>>,
}
