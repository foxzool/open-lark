use std::io::Read;

use bytes::Bytes;
use log::debug;
use serde_json::json;
use ureq::Response;

use crate::core::api_resp::ApiResp;
use crate::core::http::Transport;
use crate::core::SDKResult;
use crate::service::im::v1::message::{MessageCardTemplate, MessageText};
use crate::service::im::v1::message::SendMessageTrait;

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

impl CustomBot {
    pub fn send_message(&self, message: impl SendMessageTrait) -> SDKResult<ApiResp> {
        let json = json!({
            "msg_type": message.msg_type(),
            "content": message.content()
        });

        Transport::do_send(
            self.client.post(&self.webhook_url),
            json.to_string().as_bytes(),
        )
    }

    /// 发送飞书卡片消息， 因为自定义机器人发送飞书卡片消息的格式比较特殊，所以单独提供一个方法
    pub fn send_card(&self, message: MessageCardTemplate) -> SDKResult<ApiResp> {
        let json = json!({
            "msg_type": message.msg_type(),
            "card": message.content()
        });

        Transport::do_send(
            self.client.post(&self.webhook_url),
            json.to_string().as_bytes(),
        )
    }
}
