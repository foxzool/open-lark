use log::debug;
use serde::Serialize;
use ureq::Response;

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
    client: ureq::Agent,
}

impl CustomBot {
    pub fn new(webhook_url: String, secret: Option<String>) -> Self {
        CustomBot {
            webhook_url,
            secret,
            client: ureq::Agent::new(),
        }
    }
}

impl LarkBot for CustomBot {
    fn send_raw_message(&self, body: impl Serialize + Send) -> Response  {
        self.client
            .post(&self.webhook_url)
            .send_json(&body)
            .unwrap()
    }

    fn send_message(&self, message: impl MessageTrait + Send) -> Response  {
        let body = serde_json::json!({
           "msg_type": message.message_type(),
            "content": message
        });

        debug!("{}", serde_json::to_string_pretty(&body).unwrap());

        self.client
            .post(&self.webhook_url)
            .send_json(&body)
            .unwrap()
    }
}
