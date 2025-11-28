use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct FileSubscription {
    service: Arc<DocsService>,
}

impl FileSubscription {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/get
    pub async fn get_open_apis_drive_v1_files_by_file_token_subscriptions_by_subscription_id(&self, file_token: impl AsRef<str>, subscription_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":subscription_id", subscription_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/create
    pub async fn post_open_apis_drive_v1_files_by_file_token_subscriptions(&self, file_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/subscriptions".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/patch
    pub async fn patch_open_apis_drive_v1_files_by_file_token_subscriptions_by_subscription_id(&self, file_token: impl AsRef<str>, subscription_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":subscription_id", subscription_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
