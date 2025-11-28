pub mod reply;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct FileComment {
    service: Arc<DocsService>,
}

impl FileComment {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/CommentAPI/list
    pub async fn get_open_apis_drive_v1_files_by_file_token_comments(&self, file_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/comments".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/CommentAPI/batch_query
    pub async fn post_open_apis_drive_v1_files_by_file_token_comments_batch_query(&self, file_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/comments/batch_query".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/CommentAPI/patch
    pub async fn patch_open_apis_drive_v1_files_by_file_token_comments_by_comment_id(&self, file_token: impl AsRef<str>, comment_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/comments/:comment_id".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":comment_id", comment_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/CommentAPI/create
    pub async fn post_open_apis_drive_v1_files_by_file_token_comments(&self, file_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/comments".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/CommentAPI/get
    pub async fn get_open_apis_drive_v1_files_by_file_token_comments_by_comment_id(&self, file_token: impl AsRef<str>, comment_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/comments/:comment_id".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":comment_id", comment_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
