use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct CustomAttr {
    service: Arc<CommunicationService>,
}

impl CustomAttr {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/custom_attr/list
    pub async fn get_open_apis_contact_v3_custom_attrs(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/custom_attrs".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
