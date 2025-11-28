use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Default {
    service: Arc<DocsService>,
}

impl Default {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta
    pub async fn get_open_apis_drive_explorer_v2_root_folder_meta(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/drive/explorer/v2/root_folder/meta".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-folder-meta
    pub async fn get_open_apis_drive_explorer_v2_folder_by_foldertoken_meta(&self, folder_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/explorer/v2/folder/:folder_token/meta".to_string();
        path = path.replace(":folder_token", folder_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create-online-document
    pub async fn post_open_apis_drive_explorer_v2_file_by_foldertoken(&self, folder_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/explorer/v2/file/:folder_token".to_string();
        path = path.replace(":folder_token", folder_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-sheet
    pub async fn delete_open_apis_drive_explorer_v2_file_spreadsheets_by_spreadsheettoken(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/explorer/v2/file/spreadsheets/:spreadsheet_token".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/copy-a-doc-or-sheet
    pub async fn post_open_apis_drive_explorer_v2_file_copy_files_by_filetoken(&self, file_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/explorer/v2/file/copy/files/:file_token".to_string();
        path = path.replace(":file_token", file_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-a-doc
    pub async fn delete_open_apis_drive_explorer_v2_file_docs_by_doctoken(&self, doc_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/explorer/v2/file/docs/:doc_token".to_string();
        path = path.replace(":doc_token", doc_token.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/get-folder-children
    pub async fn get_open_apis_drive_explorer_v2_folder_by_foldertoken_children(&self, folder_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/explorer/v2/folder/:folder_token/children".to_string();
        path = path.replace(":folder_token", folder_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/create-a-new-folder
    pub async fn post_open_apis_drive_explorer_v2_folder_by_foldertoken(&self, folder_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/explorer/v2/folder/:folder_token".to_string();
        path = path.replace(":folder_token", folder_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
