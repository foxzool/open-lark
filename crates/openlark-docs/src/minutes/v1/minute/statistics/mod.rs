use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct MinuteStatistics {
    service: Arc<DocsService>,
}

impl MinuteStatistics {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get
    pub async fn get_open_apis_minutes_v1_minutes_by_minute_token_statistics(&self, minute_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/minutes/v1/minutes/:minute_token/statistics".to_string();
        path = path.replace(":minute_token", minute_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
