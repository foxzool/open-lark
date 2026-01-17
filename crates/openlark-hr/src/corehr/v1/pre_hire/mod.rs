use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct PreHire {
    service: Arc<HrService>,
}

impl PreHire {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/pre_hire/get
    pub async fn get_open_apis_corehr_v1_pre_hires_by_pre_hire_id(
        &self,
        pre_hire_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/pre_hires/:pre_hire_id".to_string();
        path = path.replace(":pre_hire_id", pre_hire_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/pre_hire/list
    pub async fn get_open_apis_corehr_v1_pre_hires(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/pre_hires".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/pre_hire/patch
    pub async fn patch_open_apis_corehr_v1_pre_hires_by_pre_hire_id(
        &self,
        pre_hire_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/pre_hires/:pre_hire_id".to_string();
        path = path.replace(":pre_hire_id", pre_hire_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/pre_hire/delete
    pub async fn delete_open_apis_corehr_v1_pre_hires_by_pre_hire_id(
        &self,
        pre_hire_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/pre_hires/:pre_hire_id".to_string();
        path = path.replace(":pre_hire_id", pre_hire_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
