use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct UserApproval {
    service: Arc<HrService>,
}

impl UserApproval {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/query
    pub async fn post_open_apis_attendance_v1_user_approvals_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/user_approvals/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/create
    pub async fn post_open_apis_attendance_v1_user_approvals(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/user_approvals".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
