use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct LeaveAccrualRecord {
    service: Arc<HrService>,
}

impl LeaveAccrualRecord {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/leave_accrual_record/patch
    pub async fn patch_open_apis_attendance_v1_leave_accrual_record_by_leave_id(
        &self,
        leave_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/leave_accrual_record/:leave_id".to_string();
        path = path.replace(":leave_id", leave_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
