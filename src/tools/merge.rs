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

use crate::client::{GrampsClient, Result};

async fn merge_simple(
    client: &GrampsClient,
    resource: &str,
    phoenix: &str,
    titanic: &str,
) -> Result<()> {
    let path = format!("/api/{resource}/{phoenix}/merge/{titanic}");
    client
        .post::<_, serde_json::Value>(&path, &serde_json::json!({}))
        .await?;
    Ok(())
}

pub async fn merge_person(
    client: &GrampsClient,
    phoenix: &str,
    titanic: &str,
    family_merger: bool,
) -> Result<()> {
    let path = format!("/api/people/{phoenix}/merge/{titanic}");
    client
        .post::<_, serde_json::Value>(
            &path,
            &serde_json::json!({ "family_merger": family_merger }),
        )
        .await?;
    Ok(())
}

pub async fn merge_family(
    client: &GrampsClient,
    phoenix: &str,
    titanic: &str,
    phoenix_father_handle: Option<&str>,
    phoenix_mother_handle: Option<&str>,
) -> Result<()> {
    let path = format!("/api/families/{phoenix}/merge/{titanic}");
    client
        .post::<_, serde_json::Value>(
            &path,
            &serde_json::json!({
                "phoenix_father_handle": phoenix_father_handle,
                "phoenix_mother_handle": phoenix_mother_handle,
            }),
        )
        .await?;
    Ok(())
}

pub async fn merge_citation(client: &GrampsClient, phoenix: &str, titanic: &str) -> Result<()> {
    merge_simple(client, "citations", phoenix, titanic).await
}

pub async fn merge_event(client: &GrampsClient, phoenix: &str, titanic: &str) -> Result<()> {
    merge_simple(client, "events", phoenix, titanic).await
}

pub async fn merge_media(client: &GrampsClient, phoenix: &str, titanic: &str) -> Result<()> {
    merge_simple(client, "media", phoenix, titanic).await
}

pub async fn merge_note(client: &GrampsClient, phoenix: &str, titanic: &str) -> Result<()> {
    merge_simple(client, "notes", phoenix, titanic).await
}

pub async fn merge_place(client: &GrampsClient, phoenix: &str, titanic: &str) -> Result<()> {
    merge_simple(client, "places", phoenix, titanic).await
}

pub async fn merge_repository(client: &GrampsClient, phoenix: &str, titanic: &str) -> Result<()> {
    merge_simple(client, "repositories", phoenix, titanic).await
}

pub async fn merge_source(client: &GrampsClient, phoenix: &str, titanic: &str) -> Result<()> {
    merge_simple(client, "sources", phoenix, titanic).await
}
