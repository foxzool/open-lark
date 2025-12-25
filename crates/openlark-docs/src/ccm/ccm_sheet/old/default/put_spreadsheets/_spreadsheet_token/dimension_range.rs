//! 更新行列
//!
//! docPath: /document/ukTMukTMukTM/uYjMzUjL2IzM14iNyMTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/update-rows-or-columns

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
pub struct UpdateDimensionRangeRequest {
    pub dimension: DimensionUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionUpdate {
    pub sheetId: String,
    pub majorDimension: Option<String>,
    pub startIndex: Option<i32>,
    pub endIndex: Option<i32>,
    pub visible: Option<bool>,
    pub fixedSize: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDimensionRangeResponse {}

impl ApiResponseTrait for UpdateDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新行列
pub async fn dimension_range(
    spreadsheet_token: String,
    request: UpdateDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<UpdateDimensionRangeResponse> {
    let api_endpoint = CcmSheetApiOld::DimensionRangeUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<UpdateDimensionRangeResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&request, "更新行列")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新行列")
}
