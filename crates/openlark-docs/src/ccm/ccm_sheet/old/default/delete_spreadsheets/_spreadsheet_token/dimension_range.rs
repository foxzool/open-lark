//! 删除行列
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/-delete-rows-or-columns

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
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
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
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
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.dimension.sheetId, "sheetId 不能为空");
    validate_required!(request.dimension.majorDimension, "majorDimension 不能为空");
    if request.dimension.majorDimension != "ROWS" && request.dimension.majorDimension != "COLUMNS" {
        return Err(openlark_core::error::validation_error(
            "majorDimension",
            "majorDimension 必须为 ROWS 或 COLUMNS",
        ));
    }
    if request.dimension.startIndex <= 0 {
        return Err(openlark_core::error::validation_error(
            "startIndex",
            "startIndex 必须 >= 1",
        ));
    }
    if request.dimension.endIndex < request.dimension.startIndex {
        return Err(openlark_core::error::validation_error(
            "endIndex",
            "endIndex 必须 >= startIndex",
        ));
    }
    if request.dimension.endIndex - request.dimension.startIndex + 1 > 5000 {
        return Err(openlark_core::error::validation_error(
            "endIndex",
            "单次操作不超过 5000 行或列",
        ));
    }

    let api_endpoint = CcmSheetApiOld::DimensionRangeDelete(spreadsheet_token);
    let mut api_request: ApiRequest<DeleteDimensionRangeResponse> =
        ApiRequest::delete(&api_endpoint.to_url()).body(serialize_params(&request, "删除行列")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除行列")
}
