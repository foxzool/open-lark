use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct SpreadsheetSheetFilter {
    service: Arc<DocsService>,
}

impl SpreadsheetSheetFilter {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/create
    pub async fn post_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/update
    pub async fn put_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/get
    pub async fn get_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/delete
    pub async fn delete_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
