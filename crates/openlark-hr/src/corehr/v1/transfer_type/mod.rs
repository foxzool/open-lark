use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct TransferType {
    service: Arc<HrService>,
}

impl TransferType {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/job_change/query
    pub async fn get_open_apis_corehr_v1_transfer_types_query(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/transfer_types/query".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
