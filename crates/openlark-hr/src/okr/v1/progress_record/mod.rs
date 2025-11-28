use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ProgressRecord {
    service: Arc<HrService>,
}

impl ProgressRecord {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/okr-v1/progress_record/create
    pub async fn post_open_apis_okr_v1_progress_records(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/okr/v1/progress_records".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/okr-v1/progress_record/delete
    pub async fn delete_open_apis_okr_v1_progress_records_by_progress_id(&self, progress_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/okr/v1/progress_records/:progress_id".to_string();
        path = path.replace(":progress_id", progress_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/okr-v1/progress_record/update
    pub async fn put_open_apis_okr_v1_progress_records_by_progress_id(&self, progress_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/okr/v1/progress_records/:progress_id".to_string();
        path = path.replace(":progress_id", progress_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/okr-v1/progress_record/get
    pub async fn get_open_apis_okr_v1_progress_records_by_progress_id(&self, progress_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/okr/v1/progress_records/:progress_id".to_string();
        path = path.replace(":progress_id", progress_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
