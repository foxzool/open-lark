use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct SpreadsheetSheetFilterViewCondition {
    service: Arc<DocsService>,
}

impl SpreadsheetSheetFilterViewCondition {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/create
    pub async fn post_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter_views_by_filter_view_id_conditions(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, filter_view_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":filter_view_id", filter_view_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/update
    pub async fn put_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter_views_by_filter_view_id_conditions_by_condition_id(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, filter_view_id: impl AsRef<str>, condition_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":filter_view_id", filter_view_id.as_ref());
        path = path.replace(":condition_id", condition_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/query
    pub async fn get_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter_views_by_filter_view_id_conditions_query(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, filter_view_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/query".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":filter_view_id", filter_view_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/get
    pub async fn get_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter_views_by_filter_view_id_conditions_by_condition_id(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, filter_view_id: impl AsRef<str>, condition_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":filter_view_id", filter_view_id.as_ref());
        path = path.replace(":condition_id", condition_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/delete
    pub async fn delete_open_apis_sheets_v3_spreadsheets_by_spreadsheet_token_sheets_by_sheet_id_filter_views_by_filter_view_id_conditions_by_condition_id(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, filter_view_id: impl AsRef<str>, condition_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":filter_view_id", filter_view_id.as_ref());
        path = path.replace(":condition_id", condition_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
