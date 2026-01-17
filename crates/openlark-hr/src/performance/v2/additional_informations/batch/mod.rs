use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct AdditionalInformationsBatch {
    service: Arc<HrService>,
}

impl AdditionalInformationsBatch {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/performance-v1/review_config/semester_activity/additional_information/delete
    pub async fn delete_open_apis_performance_v2_additional_informations_batch(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/performance/v2/additional_informations/batch".to_string();
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
