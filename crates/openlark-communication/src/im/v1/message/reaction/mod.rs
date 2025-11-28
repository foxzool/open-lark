use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct MessageReaction {
    service: Arc<CommunicationService>,
}

impl MessageReaction {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message-reaction/create
    pub async fn post_open_apis_im_v1_messages_by_message_id_reactions(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/reactions".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message-reaction/list
    pub async fn get_open_apis_im_v1_messages_by_message_id_reactions(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/reactions".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message-reaction/delete
    pub async fn delete_open_apis_im_v1_messages_by_message_id_reactions_by_reaction_id(&self, message_id: impl AsRef<str>, reaction_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/reactions/:reaction_id".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        path = path.replace(":reaction_id", reaction_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
