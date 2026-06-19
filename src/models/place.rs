// Copyright 2026 Alexey Chernyshov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
