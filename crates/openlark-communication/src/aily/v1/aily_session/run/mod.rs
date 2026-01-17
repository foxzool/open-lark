use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AilySessionRun {
    service: Arc<CommunicationService>,
}

impl AilySessionRun {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session-run/create
    pub async fn post_open_apis_aily_v1_sessions_by_aily_session_id_runs(&self, aily_session_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id/runs".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session-run/get
    pub async fn get_open_apis_aily_v1_sessions_by_aily_session_id_runs_by_run_id(&self, aily_session_id: impl AsRef<str>, run_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id/runs/:run_id".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        path = path.replace(":run_id", run_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session-run/list
    pub async fn get_open_apis_aily_v1_sessions_by_aily_session_id_runs(&self, aily_session_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id/runs".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session-run/cancel
    pub async fn post_open_apis_aily_v1_sessions_by_aily_session_id_runs_by_run_id_cancel(&self, aily_session_id: impl AsRef<str>, run_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id/runs/:run_id/cancel".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        path = path.replace(":run_id", run_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
