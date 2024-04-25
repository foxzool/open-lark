use serde::Serialize;

use crate::bot::LarkBot;
use crate::message::MessageTrait;

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

    fn send_message(&self, message: impl MessageTrait) {
        let body = serde_json::json!({
           "msg_type": message.message_type(),
            "content": message
        });

        println!("{}", serde_json::to_string_pretty(&body).unwrap());

        self.client
            .post(&self.webhook_url)
            .json(&body)
            .send()
            .unwrap();
    }
}
