pub mod filter;
pub mod filter_view;
pub mod float_image;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct SpreadsheetSheet {
    service: Arc<DocsService>,
}

impl SpreadsheetSheet {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/query
    pub async fn get_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_query(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/query".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/get
    pub async fn get_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/move_dimension
    pub async fn post_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_move_dimension(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/move_dimension".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/find
    pub async fn post_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_find(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/find".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/replace
    pub async fn post_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_replace(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/replace".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
