pub mod citation;
pub mod event;
pub mod family;
pub mod note;
pub mod person;
pub mod place;
pub mod repository;
pub mod source;
pub mod tag;

use serde::{Deserialize, Serialize};

/// Gramps object handle (opaque string ID).
pub type Handle = String;

/// A name-value date used in many Gramps objects.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrampsDate {
    pub modifier: Option<serde_json::Value>,
    pub quality: Option<serde_json::Value>,
    pub dateval: Option<serde_json::Value>,
    pub text: Option<String>,
    pub sortval: Option<i64>,
    pub newyear: Option<i32>,
}
