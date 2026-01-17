use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct WebsiteChannel {
    service: Arc<HrService>,
}

impl WebsiteChannel {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/create-2
    pub async fn post_open_apis_hire_v1_websites_by_website_id_channels(
        &self,
        website_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/websites/:website_id/channels".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/delete
    pub async fn delete_open_apis_hire_v1_websites_by_website_id_channels_by_channel_id(
        &self,
        website_id: impl AsRef<str>,
        channel_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/websites/:website_id/channels/:channel_id".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        path = path.replace(":channel_id", channel_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/update
    pub async fn put_open_apis_hire_v1_websites_by_website_id_channels_by_channel_id(
        &self,
        website_id: impl AsRef<str>,
        channel_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/websites/:website_id/channels/:channel_id".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        path = path.replace(":channel_id", channel_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/list-3
    pub async fn get_open_apis_hire_v1_websites_by_website_id_channels(
        &self,
        website_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/websites/:website_id/channels".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
