use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct FileVersion {
    service: Arc<DocsService>,
}

impl FileVersion {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/create
    pub async fn post_open_apis_drive_v1_files_by_file_token_versions(&self, file_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/versions".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/list
    pub async fn get_open_apis_drive_v1_files_by_file_token_versions(&self, file_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/versions".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/get
    pub async fn get_open_apis_drive_v1_files_by_file_token_versions_by_version_id(&self, file_token: impl AsRef<str>, version_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/versions/:version_id".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":version_id", version_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/delete
    pub async fn delete_open_apis_drive_v1_files_by_file_token_versions_by_version_id(&self, file_token: impl AsRef<str>, version_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/versions/:version_id".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":version_id", version_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
