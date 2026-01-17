use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ChatButton {
    service: Arc<CommunicationService>,
}

impl ChatButton {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/im-v2/groups-bots/update
    pub async fn put_open_apis_im_v2_chat_button(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v2/chat_button".to_string();
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
