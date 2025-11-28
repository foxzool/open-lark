use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct FileCommentReply {
    service: Arc<DocsService>,
}

impl FileCommentReply {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/CommentAPI/list-2
    pub async fn get_open_apis_drive_v1_files_by_file_token_comments_by_comment_id_replies(&self, file_token: impl AsRef<str>, comment_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":comment_id", comment_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/CommentAPI/update
    pub async fn put_open_apis_drive_v1_files_by_file_token_comments_by_comment_id_replies_by_reply_id(&self, file_token: impl AsRef<str>, comment_id: impl AsRef<str>, reply_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":comment_id", comment_id.as_ref());
        path = path.replace(":reply_id", reply_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/CommentAPI/delete
    pub async fn delete_open_apis_drive_v1_files_by_file_token_comments_by_comment_id_replies_by_reply_id(&self, file_token: impl AsRef<str>, comment_id: impl AsRef<str>, reply_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        path = path.replace(":comment_id", comment_id.as_ref());
        path = path.replace(":reply_id", reply_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
