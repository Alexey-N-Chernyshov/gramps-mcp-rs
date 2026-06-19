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

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing or invalid environment variable: {0}")]
    Env(#[from] envy::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub gramps_api_url: String,
    pub gramps_username: String,
    pub gramps_password: String,
    #[serde(default)]
    pub gramps_readonly: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(envy::from_env::<Config>()?)
    }
}
