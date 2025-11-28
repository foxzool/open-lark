use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct SocialInsurance {
    service: Arc<HrService>,
}

impl SocialInsurance {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/social_insurance/list
    pub async fn get_open_apis_compensation_v1_social_insurances(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/compensation/v1/social_insurances".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
