use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ChatMembers {
    service: Arc<CommunicationService>,
}

impl ChatMembers {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-member/create
    pub async fn post_open_apis_im_v1_chats_by_chat_id_members(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/members".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-member/me_join
    pub async fn patch_open_apis_im_v1_chats_by_chat_id_members_me_join(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/members/me_join".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-member/delete
    pub async fn delete_open_apis_im_v1_chats_by_chat_id_members(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/members".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-member/get
    pub async fn get_open_apis_im_v1_chats_by_chat_id_members(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/members".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-member/is_in_chat
    pub async fn get_open_apis_im_v1_chats_by_chat_id_members_is_in_chat(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/members/is_in_chat".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
