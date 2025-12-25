//! 删除行列
//!
//! docPath: /document/ukTMukTMukTM/ucjMzUjL3IzM14yNyMTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/-delete-rows-or-columns

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
pub struct DeleteDimensionRangeRequest {
    pub dimension: DimensionDelete,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionDelete {
    pub sheetId: String,
    pub majorDimension: Option<String>,
    pub startIndex: Option<i32>,
    pub endIndex: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDimensionRangeResponse {
    pub delCount: i32,
    pub majorDimension: String,
}

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除行列
pub async fn dimension_range(
    spreadsheet_token: String,
    request: DeleteDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<DeleteDimensionRangeResponse> {
    let api_endpoint = CcmSheetApiOld::DimensionRangeDelete(spreadsheet_token);
    let mut api_request: ApiRequest<DeleteDimensionRangeResponse> =
        ApiRequest::delete(&api_endpoint.to_url()).body(serialize_params(&request, "删除行列")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除行列")
}
