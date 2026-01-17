use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ChatModeration {
    service: Arc<CommunicationService>,
}

impl ChatModeration {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/update
    pub async fn put_open_apis_im_v1_chats_by_chat_id_moderation(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/moderation".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/get
    pub async fn get_open_apis_im_v1_chats_by_chat_id_moderation(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/moderation".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
