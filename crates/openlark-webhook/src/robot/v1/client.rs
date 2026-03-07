use crate::common::error::{Result, WebhookError};
use crate::robot::v1::send::SendWebhookMessageResponse;
use serde_json::json;

/// Webhook 客户端
pub struct WebhookClient {
    client: reqwest::Client,
}

impl WebhookClient {
    /// 创建新的 Webhook 客户端
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// 发送原始 JSON 负载到 Webhook
    pub async fn send(
        &self,
        webhook_url: &str,
        payload: serde_json::Value,
    ) -> Result<SendWebhookMessageResponse> {
        let response = self
            .client
            .post(webhook_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| WebhookError::Http(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(WebhookError::Http(format!("HTTP error: {}", status)));
        }

        let body = response
            .text()
            .await
            .map_err(|e| WebhookError::Http(e.to_string()))?;

        let result: SendWebhookMessageResponse = serde_json::from_str(&body)?;
        Ok(result)
    }

    /// 发送文本消息
    pub async fn send_text(
        &self,
        webhook_url: &str,
        text: String,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "text",
            "content": {
                "text": text
            }
        });
        self.send(webhook_url, payload).await
    }

    /// 发送富文本消息
    pub async fn send_post(
        &self,
        webhook_url: &str,
        post: String,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "post",
            "content": {
                "post": post
            }
        });
        self.send(webhook_url, payload).await
    }

    /// 发送图片消息
    pub async fn send_image(
        &self,
        webhook_url: &str,
        image_key: String,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "image",
            "content": {
                "image_key": image_key
            }
        });
        self.send(webhook_url, payload).await
    }

    /// 发送文件消息
    pub async fn send_file(
        &self,
        webhook_url: &str,
        file_key: String,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "file",
            "content": {
                "file_key": file_key
            }
        });
        self.send(webhook_url, payload).await
    }

    /// 发送卡片消息（需要启用 card feature）
    #[cfg(feature = "card")]
    pub async fn send_card(
        &self,
        webhook_url: &str,
        card: serde_json::Value,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "interactive",
            "content": {
                "card": card
            }
        });
        self.send(webhook_url, payload).await
    }
}

impl Default for WebhookClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_client_creation() {
        let _client = WebhookClient::new();
        let _default_client = WebhookClient::default();
    }

    #[tokio::test]
    async fn test_send_text_message_construction() {
        let client = WebhookClient::new();
        // Test that the method exists and can be called
        // (actual HTTP call would require mocking)
        let _client_ref = &client;
    }
}
