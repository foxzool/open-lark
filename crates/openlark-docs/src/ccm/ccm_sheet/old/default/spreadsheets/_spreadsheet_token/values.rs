//! 向单个范围写入数据
//!
//! docPath: /document/ukTMukTMukTM/uAjMzUjLwIzM14CMyMTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range

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
pub struct UpdateValuesRequest {
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateValuesResponse {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
    pub revision: i32,
}

impl ApiResponseTrait for UpdateValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 向单个范围写入数据
pub async fn values(
    spreadsheet_token: String,
    request: UpdateValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<UpdateValuesResponse> {
    let api_endpoint = CcmSheetApiOld::Values(spreadsheet_token);
    let mut api_request: ApiRequest<UpdateValuesResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&request, "向单个范围写入数据")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "向单个范围写入数据")
}

pub mod _range;
pub use _range::*;
