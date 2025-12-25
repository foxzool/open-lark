//! 获取表格元数据
//!
//! docPath: /document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/obtain-spreadsheet-metadata

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSpreadsheetMetaRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSpreadsheetMetaResponse {
    pub spreadsheet_token: String,
    pub properties: SpreadsheetProperties,
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpreadsheetProperties {
    pub title: String,
    pub owner_user: Option<i64>,
    pub sheet_count: i32,
    pub revision: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Sheet {
    pub sheet_id: String,
    pub title: String,
    pub index: i32,
    pub row_count: i32,
    pub column_count: i32,
    pub frozen_row_count: i32,
    pub frozen_col_count: i32,
}

impl ApiResponseTrait for GetSpreadsheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表格元数据
pub async fn metainfo(
    spreadsheet_token: String,
    _request: GetSpreadsheetMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetSpreadsheetMetaResponse> {
    let api_endpoint = CcmSheetApiOld::Metainfo(spreadsheet_token);
    let mut api_request: ApiRequest<GetSpreadsheetMetaResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取表格元数据")
}
