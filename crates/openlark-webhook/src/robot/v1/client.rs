use crate::common::error::Result;

pub struct WebhookClient;

impl WebhookClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn send(&self, webhook_url: &str, payload: serde_json::Value) -> Result<()> {
        let _response = reqwest::Client::new()
            .post(webhook_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| crate::common::error::WebhookError::Http(e.to_string()))?;

        Ok(())
    }
}

impl Default for WebhookClient {
    fn default() -> Self {
        Self::new()
    }
}
