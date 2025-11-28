use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ChangeReason {
    service: Arc<HrService>,
}

impl ChangeReason {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/compensation-setting-and-adjustment/list
    pub async fn get_open_apis_compensation_v1_change_reasons(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/compensation/v1/change_reasons".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
