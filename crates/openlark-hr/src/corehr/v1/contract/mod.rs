use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Contract {
    service: Arc<HrService>,
}

impl Contract {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/contract/create
    pub async fn post_open_apis_corehr_v1_contracts(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/contracts".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/contract/patch
    pub async fn patch_open_apis_corehr_v1_contracts_by_contract_id(
        &self,
        contract_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/contracts/:contract_id".to_string();
        path = path.replace(":contract_id", contract_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/contract/delete
    pub async fn delete_open_apis_corehr_v1_contracts_by_contract_id(
        &self,
        contract_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/contracts/:contract_id".to_string();
        path = path.replace(":contract_id", contract_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/contract/get
    pub async fn get_open_apis_corehr_v1_contracts_by_contract_id(
        &self,
        contract_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/contracts/:contract_id".to_string();
        path = path.replace(":contract_id", contract_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/contract/list
    pub async fn get_open_apis_corehr_v1_contracts(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/contracts".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
