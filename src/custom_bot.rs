#[cfg(feature = "im")]
use base64::{prelude::BASE64_STANDARD, Engine};
#[cfg(feature = "im")]
use hmac::{Hmac, Mac};
#[cfg(feature = "im")]
use serde_json::{json, Value};
#[cfg(feature = "im")]
use sha2::Sha256;

#[cfg(feature = "im")]
use crate::core::{
    api_resp::{BaseResponse, RawResponse},
    http::Transport,
    SDKResult,
};

#[cfg(feature = "im")]
use crate::service::im::v1::message::{MessageCardTemplate, SendMessageTrait};

/// 自定义机器人
///
/// [使用指南](https://open.feishu.cn/document/client-docs/bot-v3/add-custom-bot)
#[allow(dead_code)]
pub struct CustomBot<'a> {
    /// webhook 地址
    webhook_url: &'a str,
    /// 密钥
    secret: Option<&'a str>,
    client: reqwest::Client,
}

impl<'a> CustomBot<'a> {
    pub fn new(webhook_url: &'a str, secret: Option<&'a str>) -> Self {
        CustomBot {
            webhook_url,
            secret,
            client: reqwest::Client::new(),
        }
    }
}

impl CustomBot<'_> {
    #[cfg(feature = "im")]
    pub async fn send_message<T>(&self, message: T) -> SDKResult<BaseResponse<RawResponse>>
    where
        T: SendMessageTrait,
    {
        let mut json = json!({
            "msg_type": message.msg_type(),
            "content": message.content()
        });
        self.check_sign(&mut json);
        Transport::do_send(
            self.client.post(self.webhook_url),
            json.to_string().into(),
            false,
        )
        .await
    }

    /// 发送飞书卡片消息， 因为自定义机器人发送飞书卡片消息的格式比较特殊，所以单独提供一个方法
    #[cfg(feature = "im")]
    pub async fn send_card(
        &self,
        message: MessageCardTemplate,
    ) -> SDKResult<BaseResponse<RawResponse>> {
        let mut json = json!({
            "msg_type": message.msg_type(),
            "card": message.content()
        });

        self.check_sign(&mut json);

        Transport::do_send(
            self.client.post(self.webhook_url),
            json.to_string().into_bytes(),
            false,
        )
        .await
    }

    /// 如果设置了密钥，就计算签名
    #[cfg(feature = "im")]
    fn check_sign(&self, json: &mut Value) {
        if let Some(secret) = self.secret.as_ref() {
            let now = chrono::Local::now().timestamp();
            json["timestamp"] = serde_json::to_value(now).unwrap();
            let sign = CustomBot::sign(now, secret);
            json["sign"] = serde_json::to_value(sign).unwrap();
        }
    }

    /// 计算签名
    #[cfg(feature = "im")]
    fn sign(timestamp: i64, secret: &str) -> String {
        let string_to_sign = format!("{timestamp}\n{secret}");
        let hmac: Hmac<Sha256> = Hmac::new_from_slice(string_to_sign.as_bytes()).unwrap();
        let hmac_code = hmac.finalize().into_bytes();
        BASE64_STANDARD.encode(hmac_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_bot_creation_with_secret() {
        let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/test";
        let secret = Some("test_secret");

        let bot = CustomBot::new(webhook_url, secret);

        assert_eq!(bot.webhook_url, webhook_url);
        assert_eq!(bot.secret, secret);
    }

    #[test]
    fn test_custom_bot_creation_without_secret() {
        let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/test";

        let bot = CustomBot::new(webhook_url, None);

        assert_eq!(bot.webhook_url, webhook_url);
        assert!(bot.secret.is_none());
    }

    #[test]
    fn test_custom_bot_creation_with_empty_webhook() {
        let webhook_url = "";
        let secret = Some("test_secret");

        let bot = CustomBot::new(webhook_url, secret);

        assert_eq!(bot.webhook_url, "");
        assert_eq!(bot.secret, secret);
    }

    #[test]
    fn test_custom_bot_creation_with_different_urls() {
        let test_urls = [
            "https://open.feishu.cn/open-apis/bot/v2/hook/test1",
            "https://open.larksuite.com/open-apis/bot/v2/hook/test2",
            "http://localhost:8080/webhook",
            "https://example.com/hook",
        ];

        for url in &test_urls {
            let bot = CustomBot::new(url, None);
            assert_eq!(bot.webhook_url, *url);
        }
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_sign_basic() {
        let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
        let secret = "test_secret";

        let signature = CustomBot::sign(timestamp, secret);

        // Should produce a valid base64 string
        assert!(!signature.is_empty());
        assert!(base64::prelude::BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_sign_consistency() {
        let timestamp = 1609459200;
        let secret = "test_secret";

        let signature1 = CustomBot::sign(timestamp, secret);
        let signature2 = CustomBot::sign(timestamp, secret);

        // Same inputs should produce same signature
        assert_eq!(signature1, signature2);
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_sign_different_inputs() {
        let timestamp1 = 1609459200;
        let timestamp2 = 1609459201;
        let secret = "test_secret";

        let signature1 = CustomBot::sign(timestamp1, secret);
        let signature2 = CustomBot::sign(timestamp2, secret);

        // Different timestamps should produce different signatures
        assert_ne!(signature1, signature2);

        let signature3 = CustomBot::sign(timestamp1, "different_secret");

        // Different secrets should produce different signatures
        assert_ne!(signature1, signature3);
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_sign_with_empty_secret() {
        let timestamp = 1609459200;
        let secret = "";

        let signature = CustomBot::sign(timestamp, secret);

        // Should handle empty secret without panicking
        assert!(!signature.is_empty());
        assert!(base64::prelude::BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_sign_with_special_characters() {
        let timestamp = 1609459200;
        let secret = "test_secret!@#$%^&*()_+";

        let signature = CustomBot::sign(timestamp, secret);

        // Should handle special characters in secret
        assert!(!signature.is_empty());
        assert!(base64::prelude::BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_sign_with_unicode() {
        let timestamp = 1609459200;
        let secret = "测试密钥🔐";

        let signature = CustomBot::sign(timestamp, secret);

        // Should handle Unicode characters
        assert!(!signature.is_empty());
        assert!(base64::prelude::BASE64_STANDARD.decode(&signature).is_ok());
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_check_sign_with_secret() {
        let webhook_url = "https://test.webhook.url";
        let secret = Some("test_secret");
        let bot = CustomBot::new(webhook_url, secret);

        let mut json = json!({
            "msg_type": "text",
            "content": "test message"
        });

        bot.check_sign(&mut json);

        // Should add timestamp and sign fields
        assert!(json["timestamp"].is_i64());
        assert!(json["sign"].is_string());
        assert!(!json["sign"].as_str().unwrap().is_empty());
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_check_sign_without_secret() {
        let webhook_url = "https://test.webhook.url";
        let bot = CustomBot::new(webhook_url, None);

        let mut json = json!({
            "msg_type": "text",
            "content": "test message"
        });
        let original_json = json.clone();

        bot.check_sign(&mut json);

        // Should not modify JSON when no secret is set
        assert_eq!(json, original_json);
        assert!(json["timestamp"].is_null());
        assert!(json["sign"].is_null());
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_check_sign_preserves_existing_fields() {
        let webhook_url = "https://test.webhook.url";
        let secret = Some("test_secret");
        let bot = CustomBot::new(webhook_url, secret);

        let mut json = json!({
            "msg_type": "text",
            "content": "test message",
            "existing_field": "existing_value"
        });

        bot.check_sign(&mut json);

        // Should preserve existing fields
        assert_eq!(json["msg_type"], "text");
        assert_eq!(json["content"], "test message");
        assert_eq!(json["existing_field"], "existing_value");

        // Should add new fields
        assert!(json["timestamp"].is_i64());
        assert!(json["sign"].is_string());
    }

    #[test]
    fn test_custom_bot_is_send_sync() {
        // Test that CustomBot implements required traits for concurrent usage
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<CustomBot>();
        assert_sync::<CustomBot>();
    }

    #[test]
    fn test_custom_bot_lifetime() {
        let webhook_url = String::from("https://test.webhook.url");
        let secret_str = String::from("test_secret");

        // Test that CustomBot can be created with string references
        let bot = CustomBot::new(&webhook_url, Some(&secret_str));

        assert_eq!(bot.webhook_url, webhook_url.as_str());
        assert_eq!(bot.secret, Some(secret_str.as_str()));
    }

    #[test]
    fn test_custom_bot_multiple_instances() {
        let webhook_url1 = "https://test1.webhook.url";
        let webhook_url2 = "https://test2.webhook.url";
        let secret1 = Some("secret1");
        let secret2 = Some("secret2");

        let bot1 = CustomBot::new(webhook_url1, secret1);
        let bot2 = CustomBot::new(webhook_url2, secret2);

        assert_eq!(bot1.webhook_url, webhook_url1);
        assert_eq!(bot1.secret, secret1);
        assert_eq!(bot2.webhook_url, webhook_url2);
        assert_eq!(bot2.secret, secret2);

        // Should be independent instances
        assert_ne!(bot1.webhook_url, bot2.webhook_url);
        assert_ne!(bot1.secret, bot2.secret);
    }

    #[test]
    fn test_custom_bot_debug_representation() {
        let webhook_url = "https://test.webhook.url";
        let secret = Some("test_secret");
        let bot = CustomBot::new(webhook_url, secret);

        // Should be able to create debug representation without panicking
        let debug_str = format!("{:?}", bot.client);
        assert!(debug_str.contains("Client"));
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_sign_boundary_values() {
        // Test with various timestamp boundary values
        let test_cases = [
            (0, "secret"),        // Unix epoch
            (i64::MAX, "secret"), // Maximum timestamp
            (1609459200, ""),     // Empty secret
            (-1, "secret"),       // Negative timestamp (before epoch)
        ];

        for (timestamp, secret) in &test_cases {
            let signature = CustomBot::sign(*timestamp, secret);
            assert!(
                !signature.is_empty(),
                "Failed for timestamp: {}, secret: '{}'",
                timestamp,
                secret
            );
            assert!(
                base64::prelude::BASE64_STANDARD.decode(&signature).is_ok(),
                "Invalid base64 for timestamp: {}, secret: '{}'",
                timestamp,
                secret
            );
        }
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_sign_very_long_secret() {
        let timestamp = 1609459200;
        let long_secret = "a".repeat(1000); // Very long secret

        let signature = CustomBot::sign(timestamp, &long_secret);

        assert!(!signature.is_empty());
        assert!(base64::prelude::BASE64_STANDARD.decode(&signature).is_ok());
    }
}
