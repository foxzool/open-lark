//! 获取表格元数据
//!
//! docPath: /document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSpreadsheetMetaRequest {
    /// 可选：扩展字段（query 参数），例如 `protectedRange`
    #[serde(skip)]
    pub extFields: Option<String>,
    /// 可选：返回的用户 id 类型（query 参数），可选 `open_id`、`union_id`
    #[serde(skip)]
    pub user_id_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSpreadsheetMetaResponse {
    pub spreadsheetToken: String,
    pub properties: SpreadsheetProperties,
    #[serde(default)]
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpreadsheetProperties {
    pub title: String,
    /// 所有者的 user_id，仅当 `user_id_type` 为空时返回
    pub ownerUser: Option<i64>,
    /// 所有者的 id，取决于 `user_id_type` 的值，仅当 `user_id_type` 不为空时返回
    pub ownerUserID: Option<String>,
    pub sheetCount: i32,
    pub revision: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Sheet {
    pub sheetId: String,
    pub title: String,
    pub index: i32,
    pub rowCount: i32,
    pub columnCount: i32,
    pub frozenRowCount: i32,
    pub frozenColCount: i32,
    /// 合并单元格范围
    #[serde(default)]
    pub merges: Vec<MergeRange>,
    /// 保护范围（当 query 参数 `extFields=protectedRange` 时返回）
    #[serde(default)]
    pub protectedRange: Vec<ProtectedRange>,
    /// 若含有该字段，则此工作表不为表格
    pub blockInfo: Option<BlockInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MergeRange {
    pub startRowIndex: i32,
    pub startColumnIndex: i32,
    pub rowCount: i32,
    pub columnCount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedRange {
    /// 保护行列的信息，如果为保护工作表，则该字段为空
    pub dimension: Option<ProtectedRangeDimension>,
    pub protectId: String,
    pub lockInfo: Option<String>,
    pub sheetId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedRangeDimension {
    pub startIndex: i32,
    pub endIndex: i32,
    pub majorDimension: String,
    pub sheetId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BlockInfo {
    pub blockToken: String,
    pub blockType: String,
}

impl ApiResponseTrait for GetSpreadsheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表格元数据
pub async fn metainfo(
    spreadsheet_token: String,
    request: GetSpreadsheetMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetSpreadsheetMetaResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if let Some(user_id_type) = request.user_id_type.as_deref() {
        if user_id_type != "open_id" && user_id_type != "union_id" {
            return Err(openlark_core::error::validation_error(
                "user_id_type",
                "user_id_type 必须为 open_id 或 union_id",
            ));
        }
    }

    let api_endpoint = CcmSheetApiOld::Metainfo(spreadsheet_token);
    let mut api_request: ApiRequest<GetSpreadsheetMetaResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    if let Some(ext_fields) = request.extFields.as_deref() {
        if !ext_fields.trim().is_empty() {
            api_request = api_request.query("extFields", ext_fields);
        }
    }
    if let Some(user_id_type) = request.user_id_type.as_deref() {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取表格元数据")
}
