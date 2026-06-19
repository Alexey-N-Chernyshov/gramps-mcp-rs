use crate::client::{GrampsClient, Result};

pub async fn search(
    client: &GrampsClient,
    query: &str,
    object_type: Option<&str>,
) -> Result<serde_json::Value> {
    let path = match object_type {
        Some(t) => format!(
            "/api/search/?query={}&type={}",
            urlencoding::encode(query),
            urlencoding::encode(t)
        ),
        None => format!("/api/search/?query={}", urlencoding::encode(query)),
    };
    client.get(&path).await
}
