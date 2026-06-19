use super::Handle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFamilyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub father_handle: Option<Handle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mother_handle: Option<Handle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_ref_list: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_rel_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_list: Option<Vec<Handle>>,
}
