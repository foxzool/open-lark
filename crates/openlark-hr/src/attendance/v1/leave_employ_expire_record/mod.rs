use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct LeaveEmployExpireRecord {
    service: Arc<HrService>,
}

impl LeaveEmployExpireRecord {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/leave_employ_expire_record/get
    pub async fn get_open_apis_attendance_v1_leave_employ_expire_records_by_leave_id(&self, leave_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/leave_employ_expire_records/:leave_id".to_string();
        path = path.replace(":leave_id", leave_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
