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

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_sheets_batch_update(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/sheets_batch_update".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/update-sheet-properties
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_sheets_batch_update_2(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/sheets_batch_update".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/add-rows-or-columns
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_dimension_range(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/dimension_range".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/insert-rows-or-columns
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_insert_dimension_range(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/insert_dimension_range".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/update-rows-or-columns
    pub async fn put_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_dimension_range(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/dimension_range".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/-delete-rows-or-columns
    pub async fn delete_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_dimension_range(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/dimension_range".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/merge-cells
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_merge_cells(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/merge_cells".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/split-cells
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_unmerge_cells(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/unmerge_cells".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/set-cell-style
    pub async fn put_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_style(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/style".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/batch-set-cell-style
    pub async fn put_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_styles_batch_update(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/styles_batch_update".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/prepend-data
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_values_prepend(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/values_prepend".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/append-data
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_values_append(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/values_append".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-images
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_values_image(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/values_image".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-a-single-range
    pub async fn get_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_values_by_range(&self, spreadsheet_token: impl AsRef<str>, range: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/values/:range".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":range", range.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-multiple-ranges
    pub async fn get_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_values_batch_get(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/values_batch_get".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range
    pub async fn put_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_values(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/values".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-multiple-ranges
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_values_batch_update(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/values_batch_update".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/add-locked-cells
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_protected_dimension(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/protected_dimension".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/modify-protection-scopes
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_protected_range_batch_update(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/protected_range_batch_update".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/retrieve-protection-scopes
    pub async fn get_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_protected_range_batch_get(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/protected_range_batch_get".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/delete-protection-scopes
    pub async fn delete_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_protected_range_batch_del(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/protected_range_batch_del".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/set-dropdown
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_datavalidation(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/dataValidation".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/update-datavalidation
    pub async fn put_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_datavalidation_by_sheetid_by_datavalidationid(&self, spreadsheet_token: impl AsRef<str>, sheet_id: impl AsRef<str>, data_validation_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/dataValidation/:sheet_id/:data_validation_id".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        path = path.replace(":sheet_id", sheet_id.as_ref());
        path = path.replace(":data_validation_id", data_validation_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/query-datavalidation
    pub async fn get_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_datavalidation(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/dataValidation".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/delete-datavalidation
    pub async fn delete_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_datavalidation(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/dataValidation".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-set
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_condition_formats_batch_create(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/condition_formats/batch_create".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-update
    pub async fn post_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_condition_formats_batch_update(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/condition_formats/batch_update".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-get
    pub async fn get_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_condition_formats(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/condition_formats".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-delete
    pub async fn delete_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_condition_formats_batch_delete(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/condition_formats/batch_delete".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/obtain-spreadsheet-metadata
    pub async fn get_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_metainfo(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/metainfo".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/update-spreadsheet-properties
    pub async fn put_open_apis_sheets_v2_spreadsheets_by_spreadsheettoken_properties(&self, spreadsheet_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/properties".to_string();
        path = path.replace(":spreadsheet_token", spreadsheet_token.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/sheet-operation/import-spreadsheet
    pub async fn post_open_apis_sheets_v2_import(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/sheets/v2/import".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/sheet-operation/query-import-results
    pub async fn get_open_apis_sheets_v2_import_result(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/sheets/v2/import/result".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
