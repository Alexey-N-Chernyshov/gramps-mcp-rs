use crate::client::{GrampsClient, Result};

pub async fn delete_object(client: &GrampsClient, endpoint: &str, handle: &str) -> Result<()> {
    client.delete(&format!("/api/{endpoint}/{handle}")).await
}
