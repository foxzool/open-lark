pub mod aily_message;
pub mod run;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AilySession {
    service: Arc<CommunicationService>,
}

impl AilySession {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session/create
    pub async fn post_open_apis_aily_v1_sessions(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session/update
    pub async fn put_open_apis_aily_v1_sessions_by_aily_session_id(&self, aily_session_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session/get
    pub async fn get_open_apis_aily_v1_sessions_by_aily_session_id(&self, aily_session_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session/delete
    pub async fn delete_open_apis_aily_v1_sessions_by_aily_session_id(&self, aily_session_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
