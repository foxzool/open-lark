use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct UrlPreview {
    service: Arc<CommunicationService>,
}

impl UrlPreview {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/im-v1/url_preview/batch_update
    pub async fn post_open_apis_im_v2_url_previews_batch_update(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v2/url_previews/batch_update".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
