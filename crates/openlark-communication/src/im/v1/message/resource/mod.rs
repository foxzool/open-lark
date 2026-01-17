use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct MessageResource {
    service: Arc<CommunicationService>,
}

impl MessageResource {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/message/get-2
    pub async fn get_open_apis_im_v1_messages_by_message_id_resources_by_file_key(&self, message_id: impl AsRef<str>, file_key: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/messages/:message_id/resources/:file_key".to_string();
        path = path.replace(":message_id", message_id.as_ref());
        path = path.replace(":file_key", file_key.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
