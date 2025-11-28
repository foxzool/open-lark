pub mod announcement;
pub mod managers;
pub mod members;
pub mod menu_item;
pub mod menu_tree;
pub mod moderation;
pub mod tab;
pub mod top_notice;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Chat {
    service: Arc<CommunicationService>,
}

impl Chat {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/create
    pub async fn post_open_apis_im_v1_chats(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/delete
    pub async fn delete_open_apis_im_v1_chats_by_chat_id(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/update-2
    pub async fn put_open_apis_im_v1_chats_by_chat_id(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/get-2
    pub async fn get_open_apis_im_v1_chats_by_chat_id(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/list
    pub async fn get_open_apis_im_v1_chats(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/search
    pub async fn get_open_apis_im_v1_chats_search(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/search".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat/link
    pub async fn post_open_apis_im_v1_chats_by_chat_id_link(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/link".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
