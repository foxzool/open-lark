pub mod reaction;
pub mod resource;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Message {
    service: Arc<CommunicationService>,
}

impl Message {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/create
    pub async fn post_open_apis_im_v1_messages(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/reply
    pub async fn post_open_apis_im_v1_messages_by_message_id_reply(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/reply".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/update
    pub async fn put_open_apis_im_v1_messages_by_message_id(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/forward
    pub async fn post_open_apis_im_v1_messages_by_message_id_forward(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/forward".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/merge_forward
    pub async fn post_open_apis_im_v1_messages_merge_forward(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/merge_forward".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/delete
    pub async fn delete_open_apis_im_v1_messages_by_message_id(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/im-v1/message/push_follow_up
    pub async fn post_open_apis_im_v1_messages_by_message_id_push_follow_up(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/push_follow_up".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/read_users
    pub async fn get_open_apis_im_v1_messages_by_message_id_read_users(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/read_users".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/list
    pub async fn get_open_apis_im_v1_messages(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/get
    pub async fn get_open_apis_im_v1_messages_by_message_id(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/buzz-messages/urgent_app
    pub async fn patch_open_apis_im_v1_messages_by_message_id_urgent_app(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/urgent_app".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/buzz-messages/urgent_sms
    pub async fn patch_open_apis_im_v1_messages_by_message_id_urgent_sms(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/urgent_sms".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/buzz-messages/urgent_phone
    pub async fn patch_open_apis_im_v1_messages_by_message_id_urgent_phone(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/urgent_phone".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message-card/patch
    pub async fn patch_open_apis_im_v1_messages_by_message_id(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
