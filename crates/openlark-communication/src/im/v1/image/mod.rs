use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Image {
    service: Arc<CommunicationService>,
}

impl Image {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/image/create
    pub async fn post_open_apis_im_v1_images(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/images".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/im-v1/image/get
    pub async fn get_open_apis_im_v1_images_by_image_key(&self, image_key: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/images/:image_key".to_string();
        path = path.replace(":image_key", image_key.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
