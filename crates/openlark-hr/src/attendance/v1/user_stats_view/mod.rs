use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct UserStatsView {
    service: Arc<HrService>,
}

impl UserStatsView {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/update
    pub async fn put_open_apis_attendance_v1_user_stats_views_by_user_stats_view_id(&self, user_stats_view_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/user_stats_views/:user_stats_view_id".to_string();
        path = path.replace(":user_stats_view_id", user_stats_view_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query
    pub async fn post_open_apis_attendance_v1_user_stats_views_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/user_stats_views/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
