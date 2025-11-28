pub mod sheet;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Spreadsheet {
    service: Arc<DocsService>,
}

impl Spreadsheet {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/create
    pub async fn post_open_apis_sheets_v3_spreadsheets(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/sheets/v3/spreadsheets".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/patch
    pub async fn patch_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/get
    pub async fn get_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
