use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AilySessionAilyMessage {
    service: Arc<CommunicationService>,
}

impl AilySessionAilyMessage {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session-aily_message/create
    pub async fn post_open_apis_aily_v1_sessions_by_aily_session_id_messages(&self, aily_session_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id/messages".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session-aily_message/get
    pub async fn get_open_apis_aily_v1_sessions_by_aily_session_id_messages_by_aily_message_id(&self, aily_session_id: impl AsRef<str>, aily_message_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id/messages/:aily_message_id".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        path = path.replace(":aily_message_id", aily_message_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/aily_session-aily_message/list
    pub async fn get_open_apis_aily_v1_sessions_by_aily_session_id_messages(&self, aily_session_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/sessions/:aily_session_id/messages".to_string();
        path = path.replace(":aily_session_id", aily_session_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
