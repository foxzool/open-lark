use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct WebsiteJobPost {
    service: Arc<HrService>,
}

impl WebsiteJobPost {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/get
    pub async fn get_open_apis_hire_v1_websites_by_website_id_job_posts_by_job_post_id(&self, website_id: impl AsRef<str>, job_post_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/websites/:website_id/job_posts/:job_post_id".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        path = path.replace(":job_post_id", job_post_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/search
    pub async fn post_open_apis_hire_v1_websites_by_website_id_job_posts_search(&self, website_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/websites/:website_id/job_posts/search".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/list-2
    pub async fn get_open_apis_hire_v1_websites_by_website_id_job_posts(&self, website_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/websites/:website_id/job_posts".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
