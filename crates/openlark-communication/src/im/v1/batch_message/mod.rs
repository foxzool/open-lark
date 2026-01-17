use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct BatchMessage {
    service: Arc<CommunicationService>,
}

impl BatchMessage {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/batch_message/delete
    pub async fn delete_open_apis_im_v1_batch_messages_by_batch_message_id(&self, batch_message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/batch_messages/:batch_message_id".to_string();
        path = path.replace(":batch_message_id", batch_message_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/batch_message/read_user
    pub async fn get_open_apis_im_v1_batch_messages_by_batch_message_id_read_user(&self, batch_message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/batch_messages/:batch_message_id/read_user".to_string();
        path = path.replace(":batch_message_id", batch_message_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/batch_message/get_progress
    pub async fn get_open_apis_im_v1_batch_messages_by_batch_message_id_get_progress(&self, batch_message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/batch_messages/:batch_message_id/get_progress".to_string();
        path = path.replace(":batch_message_id", batch_message_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
