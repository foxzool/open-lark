use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Tag {
    service: Arc<CommunicationService>,
}

impl Tag {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/tenant-tag/create
    pub async fn post_open_apis_im_v2_tags(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v2/tags".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/tenant-tag/patch
    pub async fn patch_open_apis_im_v2_tags_by_tag_id(&self, tag_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v2/tags/:tag_id".to_string();
        path = path.replace(":tag_id", tag_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
