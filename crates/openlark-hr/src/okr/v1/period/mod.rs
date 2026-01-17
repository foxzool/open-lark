use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Period {
    service: Arc<HrService>,
}

impl Period {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/okr-v1/period/create
    pub async fn post_open_apis_okr_v1_periods(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/okr/v1/periods".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/okr-v1/period/patch
    pub async fn patch_open_apis_okr_v1_periods_by_period_id(
        &self,
        period_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/okr/v1/periods/:period_id".to_string();
        path = path.replace(":period_id", period_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/okr-v1/period/list
    pub async fn get_open_apis_okr_v1_periods(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/okr/v1/periods".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
