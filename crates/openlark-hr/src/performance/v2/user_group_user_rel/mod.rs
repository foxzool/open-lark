use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserGroupUserRel {
    service: Arc<HrService>,
}

impl UserGroupUserRel {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/performance-v1/review_config/semester_activity/user_group_user_rel/write
    pub async fn post_open_apis_performance_v2_user_group_user_rels_write(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/performance/v2/user_group_user_rels/write".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
