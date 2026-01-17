use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct TalentPool {
    service: Arc<HrService>,
}

impl TalentPool {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/candidate-management/talent_pool/batch_change_talent_pool
    pub async fn post_open_apis_hire_v1_talent_pools_by_talent_pool_id_batch_change_talent_pool(
        &self,
        talent_pool_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/talent_pools/:talent_pool_id/batch_change_talent_pool".to_string();
        path = path.replace(":talent_pool_id", talent_pool_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent_pool/search
    pub async fn get_open_apis_hire_v1_talent_pools(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/hire/v1/talent_pools".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent_pool/move_talent
    pub async fn post_open_apis_hire_v1_talent_pools_by_talent_pool_id_talent_relationship(
        &self,
        talent_pool_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/talent_pools/:talent_pool_id/talent_relationship".to_string();
        path = path.replace(":talent_pool_id", talent_pool_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
