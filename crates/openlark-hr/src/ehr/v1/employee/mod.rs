use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Employee {
    service: Arc<HrService>,
}

impl Employee {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/ehr-v1/list
    pub async fn get_open_apis_ehr_v1_employees(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/ehr/v1/employees".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
