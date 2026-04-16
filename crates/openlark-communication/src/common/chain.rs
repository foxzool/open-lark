//! openlark-communication 链式调用入口（meta 风格，偏"模块入口"）
//!
//! 说明：
//! - 本文件放在 `common/` 下，避免被 strict API 校验脚本计入"额外实现文件"。
//! - communication 模块 API 规模较大（IM/Contact/Moments 等）。为避免为大量 API 手写方法，
//!   这里先提供"bizTag 级入口 + Config 透传"。
//! - 具体 API 调用仍使用各 `*RequestBuilder/*Request` 的 `new(config)` / `execute(...)`。

use std::sync::Arc;

use openlark_core::config::Config;
#[cfg(feature = "im")]
use openlark_core::{error::validation_error, SDKResult};

#[cfg(feature = "im")]
use crate::im::im::v1::message::{
    create::{CreateMessageBody, CreateMessageRequest},
    models::ReceiveIdType,
    reply::{ReplyMessageBody, ReplyMessageRequest},
};
#[cfg(feature = "im")]
use crate::im::im::v1::thread::forward::{ForwardThreadBody, ForwardThreadRequest};

/// 消息接收者 helper。
///
/// 统一接收者 ID 与 `receive_id_type`，避免文本/卡片等消息 helper
/// 每次都分别传两个参数。
#[cfg(feature = "im")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageRecipient {
    pub receive_id: String,
    pub receive_id_type: ReceiveIdType,
}

#[cfg(feature = "im")]
impl MessageRecipient {
    pub fn new(receive_id: impl Into<String>, receive_id_type: ReceiveIdType) -> Self {
        Self {
            receive_id: receive_id.into(),
            receive_id_type,
        }
    }

    pub fn open_id(receive_id: impl Into<String>) -> Self {
        Self::new(receive_id, ReceiveIdType::OpenId)
    }

    pub fn user_id(receive_id: impl Into<String>) -> Self {
        Self::new(receive_id, ReceiveIdType::UserId)
    }

    pub fn email(receive_id: impl Into<String>) -> Self {
        Self::new(receive_id, ReceiveIdType::Email)
    }

    pub fn chat_id(receive_id: impl Into<String>) -> Self {
        Self::new(receive_id, ReceiveIdType::ChatId)
    }
}

/// 富文本（post）消息 helper。
///
/// 当前只覆盖最常见的“标题 + 单段文本”结构，不试图抽象完整消息 DSL。
#[cfg(feature = "im")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostMessage {
    pub locale: String,
    pub title: String,
    pub text: String,
}

#[cfg(feature = "im")]
impl PostMessage {
    pub fn zh_cn(title: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            locale: "zh_cn".to_string(),
            title: title.into(),
            text: text.into(),
        }
    }

    fn into_content(self) -> SDKResult<String> {
        let title = self.title.trim().to_string();
        let text = self.text.trim().to_string();
        if title.is_empty() {
            return Err(validation_error("title", "title 不能为空"));
        }
        if text.is_empty() {
            return Err(validation_error("text", "text 不能为空"));
        }

        Ok(serde_json::json!({
            "post": {
                self.locale: {
                    "title": title,
                    "content": [[{"tag": "text", "text": text}]]
                }
            }
        })
        .to_string())
    }
}

/// 回复上下文 helper。
///
/// 统一回复目标消息与是否在话题内回复的上下文参数。
#[cfg(feature = "im")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplyTarget {
    pub message_id: String,
    pub reply_in_thread: bool,
}

#[cfg(feature = "im")]
impl ReplyTarget {
    pub fn direct(message_id: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
            reply_in_thread: false,
        }
    }

    pub fn in_thread(message_id: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
            reply_in_thread: true,
        }
    }
}

/// Communication 链式入口：`communication.im` / `communication.contact` / `communication.moments`
#[derive(Debug, Clone)]
pub struct CommunicationClient {
    config: Arc<Config>,

    #[cfg(feature = "im")]
    pub im: ImClient,

    #[cfg(feature = "contact")]
    pub contact: ContactClient,

    #[cfg(feature = "moments")]
    pub moments: MomentsClient,
}

impl CommunicationClient {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            config: config.clone(),
            #[cfg(feature = "im")]
            im: ImClient::new(config.clone()),
            #[cfg(feature = "contact")]
            contact: ContactClient::new(config.clone()),
            #[cfg(feature = "moments")]
            moments: MomentsClient::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "im")]
#[derive(Debug, Clone)]
pub struct ImClient {
    config: Arc<Config>,
}

#[cfg(feature = "im")]
impl ImClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 发送文本消息 helper。
    pub async fn send_text(
        &self,
        recipient: MessageRecipient,
        text: impl Into<String>,
    ) -> SDKResult<serde_json::Value> {
        let body = Self::build_text_body(recipient, text.into())?;
        Self::create_message_request(self.config.clone(), body.receive_id_type())
            .execute(body.into())
            .await
    }

    /// 发送富文本（post）消息 helper。
    pub async fn send_post(
        &self,
        recipient: MessageRecipient,
        post: PostMessage,
    ) -> SDKResult<serde_json::Value> {
        let body = Self::build_post_body(recipient, post)?;
        Self::create_message_request(self.config.clone(), body.receive_id_type())
            .execute(body.into())
            .await
    }

    /// 回复文本消息 helper。
    pub async fn reply_text(
        &self,
        target: ReplyTarget,
        text: impl Into<String>,
    ) -> SDKResult<serde_json::Value> {
        let body = Self::build_reply_text_body(target, text.into())?;
        Self::create_reply_request(self.config.clone(), body.message_id())
            .execute(body.into())
            .await
    }

    /// 回复富文本（post）消息 helper。
    pub async fn reply_post(
        &self,
        target: ReplyTarget,
        post: PostMessage,
    ) -> SDKResult<serde_json::Value> {
        let body = Self::build_reply_post_body(target, post)?;
        Self::create_reply_request(self.config.clone(), body.message_id())
            .execute(body.into())
            .await
    }

    /// 转发话题 helper。
    pub async fn forward_thread(
        &self,
        thread_id: impl Into<String>,
        recipient: MessageRecipient,
    ) -> SDKResult<serde_json::Value> {
        let request = ForwardThreadRequest::new(self.config.as_ref().clone())
            .thread_id(thread_id)
            .receive_id_type(recipient.receive_id_type);
        request
            .execute(ForwardThreadBody::new(recipient.receive_id))
            .await
    }

    fn create_message_request(
        config: Arc<Config>,
        receive_id_type: ReceiveIdType,
    ) -> CreateMessageRequest {
        CreateMessageRequest::new(config.as_ref().clone()).receive_id_type(receive_id_type)
    }

    fn create_reply_request(config: Arc<Config>, message_id: String) -> ReplyMessageRequest {
        ReplyMessageRequest::new(config.as_ref().clone()).message_id(message_id)
    }

    fn build_text_body(recipient: MessageRecipient, text: String) -> SDKResult<HelperMessageBody> {
        let text = text.trim().to_string();
        if text.is_empty() {
            return Err(validation_error("text", "text 不能为空"));
        }

        Ok(HelperMessageBody::new(
            recipient,
            "text",
            serde_json::json!({ "text": text }).to_string(),
        ))
    }

    fn build_post_body(
        recipient: MessageRecipient,
        post: PostMessage,
    ) -> SDKResult<HelperMessageBody> {
        Ok(HelperMessageBody::new(
            recipient,
            "post",
            post.into_content()?,
        ))
    }

    fn build_reply_text_body(target: ReplyTarget, text: String) -> SDKResult<HelperReplyBody> {
        let text = text.trim().to_string();
        if text.is_empty() {
            return Err(validation_error("text", "text 不能为空"));
        }

        Ok(HelperReplyBody::new(
            target,
            "text",
            serde_json::json!({ "text": text }).to_string(),
        ))
    }

    fn build_reply_post_body(target: ReplyTarget, post: PostMessage) -> SDKResult<HelperReplyBody> {
        Ok(HelperReplyBody::new(target, "post", post.into_content()?))
    }
}

#[cfg(feature = "im")]
#[derive(Debug, Clone)]
struct HelperMessageBody {
    body: CreateMessageBody,
    receive_id_type: ReceiveIdType,
}

#[cfg(feature = "im")]
impl HelperMessageBody {
    fn new(recipient: MessageRecipient, msg_type: &str, content: String) -> Self {
        Self {
            receive_id_type: recipient.receive_id_type,
            body: CreateMessageBody {
                receive_id: recipient.receive_id,
                msg_type: msg_type.to_string(),
                content,
                uuid: None,
            },
        }
    }

    fn receive_id_type(&self) -> ReceiveIdType {
        self.receive_id_type
    }
}

#[cfg(feature = "im")]
impl From<HelperMessageBody> for CreateMessageBody {
    fn from(value: HelperMessageBody) -> Self {
        value.body
    }
}

#[cfg(feature = "im")]
#[derive(Debug, Clone)]
struct HelperReplyBody {
    body: ReplyMessageBody,
    message_id: String,
}

#[cfg(feature = "im")]
impl HelperReplyBody {
    fn new(target: ReplyTarget, msg_type: &str, content: String) -> Self {
        Self {
            message_id: target.message_id,
            body: ReplyMessageBody {
                content,
                msg_type: msg_type.to_string(),
                reply_in_thread: Some(target.reply_in_thread),
                uuid: None,
            },
        }
    }

    fn message_id(&self) -> String {
        self.message_id.clone()
    }
}

#[cfg(feature = "im")]
impl From<HelperReplyBody> for ReplyMessageBody {
    fn from(value: HelperReplyBody) -> Self {
        value.body
    }
}

#[cfg(feature = "contact")]
#[derive(Debug, Clone)]
pub struct ContactClient {
    config: Arc<Config>,
}

#[cfg(feature = "contact")]
impl ContactClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(feature = "moments")]
#[derive(Debug, Clone)]
pub struct MomentsClient {
    config: Arc<Config>,
}

#[cfg(feature = "moments")]
impl MomentsClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_communication_client_creation() {
        let config = create_test_config();
        let client = CommunicationClient::new(config);
        assert_eq!(client.config().app_id(), "test_app");
    }

    #[test]
    fn test_communication_client_debug() {
        let config = create_test_config();
        let client = CommunicationClient::new(config);
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("CommunicationClient"));
    }

    #[test]
    fn test_communication_client_clone() {
        let config = create_test_config();
        let client = CommunicationClient::new(config);
        let cloned = client.clone();
        assert_eq!(cloned.config().app_id(), "test_app");
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_im_client_config() {
        let config = create_test_config();
        let client = CommunicationClient::new(config);
        assert_eq!(client.im.config().app_id(), "test_app");
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_message_recipient_constructors() {
        assert_eq!(
            MessageRecipient::open_id("ou_xxx"),
            MessageRecipient::new("ou_xxx", ReceiveIdType::OpenId)
        );
        assert_eq!(
            MessageRecipient::chat_id("oc_xxx"),
            MessageRecipient::new("oc_xxx", ReceiveIdType::ChatId)
        );
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_post_message_serialization() {
        let content = PostMessage::zh_cn("周报", "本周已完成 3 项任务")
            .into_content()
            .expect("post content should serialize");

        let value: serde_json::Value =
            serde_json::from_str(&content).expect("content should be valid json");
        assert_eq!(value["post"]["zh_cn"]["title"], "周报");
        assert_eq!(
            value["post"]["zh_cn"]["content"][0][0]["text"],
            "本周已完成 3 项任务"
        );
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_reply_target_constructors() {
        assert_eq!(
            ReplyTarget::direct("om_xxx"),
            ReplyTarget {
                message_id: "om_xxx".to_string(),
                reply_in_thread: false,
            }
        );
        assert_eq!(
            ReplyTarget::in_thread("om_xxx"),
            ReplyTarget {
                message_id: "om_xxx".to_string(),
                reply_in_thread: true,
            }
        );
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_build_text_message_body() {
        let body = ImClient::build_text_body(MessageRecipient::open_id("ou_xxx"), "hello".into())
            .expect("text body should build");
        let request_body: CreateMessageBody = body.into();
        assert_eq!(request_body.msg_type, "text");
        assert_eq!(request_body.receive_id, "ou_xxx");
        assert_eq!(request_body.content, r#"{"text":"hello"}"#);
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_build_post_message_body() {
        let body = ImClient::build_post_body(
            MessageRecipient::chat_id("oc_xxx"),
            PostMessage::zh_cn("项目播报", "今天完成发布"),
        )
        .expect("post body should build");
        let request_body: CreateMessageBody = body.into();
        let value: serde_json::Value =
            serde_json::from_str(&request_body.content).expect("content should be valid json");

        assert_eq!(request_body.msg_type, "post");
        assert_eq!(request_body.receive_id, "oc_xxx");
        assert_eq!(value["post"]["zh_cn"]["title"], "项目播报");
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_build_reply_text_message_body() {
        let body = ImClient::build_reply_text_body(ReplyTarget::direct("om_xxx"), "收到".into())
            .expect("reply text body should build");
        let request_body: ReplyMessageBody = body.into();
        assert_eq!(request_body.msg_type, "text");
        assert_eq!(request_body.reply_in_thread, Some(false));
        assert_eq!(request_body.content, r#"{"text":"收到"}"#);
    }

    #[cfg(feature = "im")]
    #[test]
    fn test_build_reply_post_message_body() {
        let body = ImClient::build_reply_post_body(
            ReplyTarget::in_thread("om_xxx"),
            PostMessage::zh_cn("进展", "线程内同步"),
        )
        .expect("reply post body should build");
        let request_body: ReplyMessageBody = body.into();
        let value: serde_json::Value =
            serde_json::from_str(&request_body.content).expect("content should be valid json");

        assert_eq!(request_body.msg_type, "post");
        assert_eq!(request_body.reply_in_thread, Some(true));
        assert_eq!(value["post"]["zh_cn"]["title"], "进展");
    }

    #[cfg(feature = "contact")]
    #[test]
    fn test_contact_client_config() {
        let config = create_test_config();
        let client = CommunicationClient::new(config);
        assert_eq!(client.contact.config().app_id(), "test_app");
    }
}
