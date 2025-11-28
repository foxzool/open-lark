use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct LeaveGrantingRecord {
    service: Arc<HrService>,
}

impl LeaveGrantingRecord {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/leave/create
    pub async fn post_open_apis_corehr_v1_leave_granting_records(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/leave_granting_records".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/leave/delete
    pub async fn delete_open_apis_corehr_v1_leave_granting_records_by_leave_granting_record_id(&self, leave_granting_record_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/leave_granting_records/:leave_granting_record_id".to_string();
        path = path.replace(":leave_granting_record_id", leave_granting_record_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
