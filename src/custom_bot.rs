use base64::{prelude::BASE64_STANDARD, Engine};
use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use sha2::Sha256;

use crate::core::api_resp::BaseResponse;
use crate::{
    core::{api_resp::RawResponse, http::Transport, SDKResult},
    service::im::v1::message::{MessageCardTemplate, SendMessageTrait},
};

/// 自定义机器人
///
/// [使用指南](https://open.feishu.cn/document/client-docs/bot-v3/add-custom-bot)
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

impl<'a> CustomBot<'a> {
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
    fn check_sign(&self, json: &mut Value) {
        if let Some(secret) = self.secret.as_ref() {
            let now = chrono::Local::now().timestamp();
            json["timestamp"] = serde_json::to_value(now).unwrap();
            let sign = CustomBot::sign(now, secret);
            json["sign"] = serde_json::to_value(sign).unwrap();
        }
    }

    /// 计算签名
    fn sign(timestamp: i64, secret: &str) -> String {
        let string_to_sign = format!("{}\n{}", timestamp, secret);
        let hmac: Hmac<Sha256> = Hmac::new_from_slice(string_to_sign.as_bytes()).unwrap();
        let hmac_code = hmac.finalize().into_bytes();
        BASE64_STANDARD.encode(hmac_code)
    }
}
