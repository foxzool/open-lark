use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserTaskRemedy {
    service: Arc<HrService>,
}

impl UserTaskRemedy {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/create
    pub async fn post_open_apis_attendance_v1_user_task_remedys(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/attendance/v1/user_task_remedys".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/query_user_allowed_remedys
    pub async fn post_open_apis_attendance_v1_user_task_remedys_query_user_allowed_remedys(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path =
            "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/query
    pub async fn post_open_apis_attendance_v1_user_task_remedys_query(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/attendance/v1/user_task_remedys/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
