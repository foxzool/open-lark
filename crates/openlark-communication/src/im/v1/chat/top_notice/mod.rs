use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ChatTopNotice {
    service: Arc<CommunicationService>,
}

impl ChatTopNotice {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/put_top_notice
    pub async fn post_open_apis_im_v1_chats_by_chat_id_top_notice_put_top_notice(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/top_notice/put_top_notice".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/delete_top_notice
    pub async fn post_open_apis_im_v1_chats_by_chat_id_top_notice_delete_top_notice(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/top_notice/delete_top_notice".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
