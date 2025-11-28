use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Indicator {
    service: Arc<HrService>,
}

impl Indicator {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/compensation-component-and-metric/list-3
    pub async fn get_open_apis_compensation_v1_indicators(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/compensation/v1/indicators".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
