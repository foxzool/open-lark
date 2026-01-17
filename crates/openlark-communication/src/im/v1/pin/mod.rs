use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Pin {
    service: Arc<CommunicationService>,
}

impl Pin {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/pin/create
    pub async fn post_open_apis_im_v1_pins(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/pins".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/pin/delete
    pub async fn delete_open_apis_im_v1_pins_by_message_id(&self, message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/pins/:message_id".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/pin/list
    pub async fn get_open_apis_im_v1_pins(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/pins".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
