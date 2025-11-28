use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct TripartiteAgreement {
    service: Arc<HrService>,
}

impl TripartiteAgreement {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/candidate-management/delivery-process-management/tripartite_agreement/create
    pub async fn post_open_apis_hire_v1_tripartite_agreements(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/tripartite_agreements".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/candidate-management/delivery-process-management/tripartite_agreement/list
    pub async fn get_open_apis_hire_v1_tripartite_agreements(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/tripartite_agreements".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/candidate-management/delivery-process-management/tripartite_agreement/update
    pub async fn put_open_apis_hire_v1_tripartite_agreements_by_tripartite_agreement_id(&self, tripartite_agreement_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/tripartite_agreements/:tripartite_agreement_id".to_string();
        path = path.replace(":tripartite_agreement_id", tripartite_agreement_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/candidate-management/delivery-process-management/tripartite_agreement/delete
    pub async fn delete_open_apis_hire_v1_tripartite_agreements_by_tripartite_agreement_id(&self, tripartite_agreement_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/tripartite_agreements/:tripartite_agreement_id".to_string();
        path = path.replace(":tripartite_agreement_id", tripartite_agreement_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
