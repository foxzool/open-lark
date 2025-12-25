//! 增加行列
//!
//! docPath: /document/ukTMukTMukTM/uUjMzUjL1IzM14SNyMTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/add-rows-or-columns

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
pub struct AddDimensionRangeRequest {
    pub dimension: Dimension,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    pub majorDimension: String,
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddDimensionRangeResponse {
    pub addCount: i32,
    pub majorDimension: String,
}

impl ApiResponseTrait for AddDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加行列
pub async fn dimension_range(
    spreadsheet_token: String,
    request: AddDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<AddDimensionRangeResponse> {
    let api_endpoint = CcmSheetApiOld::DimensionRange(spreadsheet_token);
    let mut api_request: ApiRequest<AddDimensionRangeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "增加行列")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "增加行列")
}
