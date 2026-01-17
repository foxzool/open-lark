use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct SpreadsheetSheetFloatImage {
    service: Arc<DocsService>,
}

impl SpreadsheetSheetFloatImage {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/create
    pub async fn post_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_float_images(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/patch
    pub async fn patch_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_float_images_by_float_image_id(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, float_image_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":float_image_id", float_image_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/get
    pub async fn get_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_float_images_by_float_image_id(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, float_image_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":float_image_id", float_image_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/query
    pub async fn get_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_float_images_query(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/query".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/delete
    pub async fn delete_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_float_images_by_float_image_id(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, float_image_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":float_image_id", float_image_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
