use serde::Serialize;

use crate::bot::LarkBot;
use crate::message::{Message, MessageType};

/// 自定义机器人
///
/// [使用指南](https://open.feishu.cn/document/client-docs/bot-v3/add-custom-bot)
pub struct CustomBot {
    /// webhook 地址
    webhook_url: String,
    /// 密钥
    secret: Option<String>,
    client: reqwest::blocking::Client,
}

impl CustomBot {
    pub fn new(webhook_url: impl ToString, secret: Option<impl ToString>) -> Self {
        CustomBot {
            webhook_url: webhook_url.to_string(),
            secret: secret.map(|s| s.to_string()),
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl LarkBot for CustomBot {
    fn send_raw_message(&self, body: impl Serialize) {
        self.client
            .post(&self.webhook_url)
            .json(&body)
            .send()
            .unwrap();
    }

    fn send_text_message(&self, content: &str) {
        let message = Message {
            msg_type: MessageType::Text,
            content: serde_json::json!({
                "text": content,
            }),
        };
        self.client
            .post(&self.webhook_url)
            .json(&message)
            .send()
            .unwrap();
    }
}
