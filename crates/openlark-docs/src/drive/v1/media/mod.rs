use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Media {
    service: Arc<DocsService>,
}

impl Media {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/upload_all
    pub async fn post_open_apis_drive_v1_medias_upload_all(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/medias/upload_all".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_prepare
    pub async fn post_open_apis_drive_v1_medias_upload_prepare(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/medias/upload_prepare".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_part
    pub async fn post_open_apis_drive_v1_medias_upload_part(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/medias/upload_part".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_finish
    pub async fn post_open_apis_drive_v1_medias_upload_finish(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/medias/upload_finish".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/download
    pub async fn get_open_apis_drive_v1_medias_by_file_token_download(&self, file_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/medias/:file_token/download".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/batch_get_tmp_download_url
    pub async fn get_open_apis_drive_v1_medias_batch_get_tmp_download_url(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/medias/batch_get_tmp_download_url".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
