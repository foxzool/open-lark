use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProcessWithdraw {
    service: Arc<HrService>,
}

impl ProcessWithdraw {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/process-form_variable_data/process-instance/update
    pub async fn put_open_apis_corehr_v2_process_withdraw_by_process_id(
        &self,
        process_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/process_withdraw/:process_id".to_string();
        path = path.replace(":process_id", process_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
