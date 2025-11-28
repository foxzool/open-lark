use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ChatTab {
    service: Arc<CommunicationService>,
}

impl ChatTab {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-tab/create
    pub async fn post_open_apis_im_v1_chats_by_chat_id_chat_tabs(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/chat_tabs".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-tab/delete_tabs
    pub async fn delete_open_apis_im_v1_chats_by_chat_id_chat_tabs_delete_tabs(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/chat_tabs/delete_tabs".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-tab/update_tabs
    pub async fn post_open_apis_im_v1_chats_by_chat_id_chat_tabs_update_tabs(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/chat_tabs/update_tabs".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-tab/sort_tabs
    pub async fn post_open_apis_im_v1_chats_by_chat_id_chat_tabs_sort_tabs(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/chat_tabs/sort_tabs".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-tab/list_tabs
    pub async fn get_open_apis_im_v1_chats_by_chat_id_chat_tabs_list_tabs(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/chat_tabs/list_tabs".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
